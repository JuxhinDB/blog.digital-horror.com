+++
title = "Working with Go in Rust via IPC"
description = "Exploring how we went about integrating Go libraries into Rust without FFI"
date = 2025-04-16
[taxonomies]
tags = ["rust", "go", "engineering"]
+++
# Introduction

Recently I have been exploring a few libraries I'd like to use to extend the set of security signals that we capture for [Have I Been Squatted](https://haveibeensquatted.com/). The overarching goal is to extract, and derive as many meaningful signals as we can, such that we are able to detect all sorts of bad actors in content-agnostic ways.

One of the libraries that we came across was [`wappalyzergo`](https://github.com/projectdiscovery/wappalyzergo)&mdash;an open-source alternative to the now closed[^1] wappalyzer project. This library is particularly neat, as it allows us to fingerprint websites without bundling in headless chromium. The only issue we had was the library being built with Go, and all of Have I Been Squatted's stack is powered by Rust. In a somewhat contrarian view, I had no interest in rewriting this library in Rust, and I wanted to have a solution that we could extend to other Go libraries in the future.

# Pesky Requirements

Before diving into the what's and the how's, it's perhaps important to give you, the reader, and understanding of the requirements and constraints that we are working with. This is important not just for clarity, but to understand in what situation this solution may (or may not!) be helpful for _you_. Let's break them down.

* **Minimal overhead**: certain parts of our analysis run within lambda functions and we need to make sure that our solution fits within the existing function. No external microservice(s).
* **Ergonomics over performance**: the integration interface needs to be pleasant to use and seemless to other crates within the workspace. This should take priority over maximising performance (more on this later).
* **Extensible**: we need to be able to add more Go libraries in the future without breaking existing interfaces.
* **Rapid development**: we need to be able to deliver this fairly quickly and get it infront of users.

Hopefully this should give you a good enough idea on what we are going for.

## FFI

My initial thought, and preferred choice was to integrate Rust & Go through some foreign function interface ([`std::ffi`](https://doc.rust-lang.org/std/ffi/index.html)). Unfortunately as it stands, we cannot simply perform a simple Go to Rust FFI, as Go does not offer a Rust-specific FFI (understandably so). The path to get there is indirect, relying on C interfaces on both ends.

You would effectively need to export functions using `cgo` and compile your code as a shared library (i.e., a C ABI). Rust, with its robust C FFI can then call these exported functions. Let us briefly go through what this might look like. Implementing this on the Go side, would look something like so.

```go
package main

/*
#include <stdlib.h>
*/
import "C"

// You'd likely want to use unsafe in real-world implementations, I'm omitting
// this to avoid compiler error.
// import "unsafe"

//export wappalyzer
func wappalyzer(a, b *C.char) *C.char {
    // Convert incoming C strings to Go strings, if you need them
    // goA := C.GoString(a)
    // goB := C.GoString(b)

    // For our demo, we'll just return a constant.
    // C.CString allocates with malloc; caller must free.
    return C.CString("cloudflare")
}

func main() {
    // A dummy main so cgo will build a shared library, not a CLI.
}
```

You can then compile the library and header files, outputting both the `.so` and `.h` files respectively.

```bash
go build -buildmode=c-shared -o libgoffi.so ffi.go
```

Besides the atrocious tagging of functions to be exported with comments required by `cgo`, this is a viable option. On the Rust side we would effectively want to link against the `.so` file that we've compiled, and call it through extern C.

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// assumes libwapp.so is on your linker path
#[link(name = "goffi", kind = "dylib")]
unsafe extern "C" {
    fn wappalyzer(a: *const c_char, b: *const c_char) -> *mut c_char;
}

fn main() {
    // Prepare some inputs:
    let a = CString::new("input1").unwrap();
    let b = CString::new("input2").unwrap();

    // SAFELY call the foreign function:
    let raw: *mut c_char = unsafe { wappalyzer(a.as_ptr(), b.as_ptr()) };

    // Convert result back into a Rust String:
    let result = unsafe {
        assert!(!raw.is_null());
        let s = CStr::from_ptr(raw).to_string_lossy().into_owned();
        // free the C-allocated string to avoid leaks:
        libc::free(raw as *mut libc::c_void);
        s
    };

    println!("wappalyzer returned: {}", result);
}
```

This is of course simpleified, and there is some surrounding work around the linker but this is the gist of it. I initially wanted to adopt this option as it was the most performant. It avoids de-/serialising data back and forth along with multiple memory copies and allocations. It's also not as complex as trying to get the Rust and Go binaries to share some common memory region.

In the end, I opted out of this option due to the added development friction added. In short, I found that this was not as pleasant to work with. Besides the `unsafe` code, we have to make sure to call `malloc` from the Rust side and include a good amount of scaffolding to things deployed end-to-end.

## `rust2go`

The fact that we had to go about this indirectly really bothered me. I kept digging and came across the [`rust2go`](https://github.com/ihciah/rust2go) project which seemed impressive. Equally impressive is the magnificent [blog post](https://en.ihcblog.com/rust2go/) by Hai Chi, which I encourage anyone interested in this area to read a few times. Unfortunately while impressive, this flips the order of integration. What I was hoping to find, was something akin to `go2rust` instead, so this option was ruled out. However, the blog post did spark a simpler idea I hadn't thought of.

## Unix Domain Socket (UDS)

The simplest approach I found was to rely on inter-process communication (IPC) via unix sockets. Rather than having C ABI be our interface boundary, we can rely on some wire protocol (e.g., protobuf) to pass messages back and forth. This is a neat solution for a number of reasons:

* Simple implementation
* Keeps boundaries separate
* Avoids memory safety issues
* Able to run locally within the same lambda runtime context

The largest tradeoff here is performance. We need to serialise and deserialize large buffers (i.e., entire website DOMs) along with multiple kernel copies from our applications to the kernel. Another tradeoff is the upfront work needed to get things hooked up. You'll need listeners on both ends, with their own read-eval loops.

I mentioned earlier in this post that performance was not as critical. The time at which we need to make these calls, we are executing other external calls (i.e., inference, database queries) concurrently. Therefore in practice, the overhead does not translate to any meaningful final end-to-end overhead. The increased work to setup listeners on both sides is offset when we continue to add more integrations

# Fleshing it out

I want to spend some time explaininghow we went about implementing this. There's quite a bit of work, so I'll only focus on key areas that are worth mentioning.

## Wire protocol

For starters, we need to have some sort of network representation of our structs and arguments, so that we can de-/serialise them in both Go and Rust. I opted for `protobuf`, purely out of experience working with it. Our `Request` type is quite simple.

```proto
// Request is the message sent from Rust to Go
message Request {
  string request_id = 1;  // Unique identifier for correlating requests with responses
  RequestType type = 2;
  oneof payload {
    WappalyzerRequest wappalyzer = 3;
    // Add more request types here
  }
}
```

The key part to note here is the `request_id`. Since we have multiple tasks emitting these requests, we cannot guarantee the order of them being processed. We use this `request_id` to tie a `Response` to a pending `Request`. The `Response` is very similar, but includes added information around the status.

```proto
// Response is the message sent from Go back to Rust
message Response {
  string request_id = 1;  // Matches the request_id from the original request
  bool success = 2;
  string error = 3;
  oneof payload {
    WappalyzerResponse wappalyzer = 4;
  }
}
```

## Go

From the Go side we should have a thin server that handles connections, and delegates the requests to the different handlers depending on the type. These snippets are reduced for clarity.

```go
// Serve starts accepting connections on the provided listener
func (s *Server) Serve(ctx context.Context, listener net.Listener) {
	var wg sync.WaitGroup

	// Create a context that can be canceled
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	for {
		// Add to wait group before starting goroutine
		wg.Add(1)
		go func(conn net.Conn) {
			defer wg.Done()
			s.handleConnection(ctx, conn)
		}(conn)
	}
}
```

At same point in our `handleConnection` depending on the request type, we can make the call to the underlying library.

```go
func (h *Handler) handleWappalyzerRequest(ctx context.Context, requestID string, req *pb.WappalyzerRequest) (*pb.Response, error) {
	technologies, err := h.wappalyzerService.Detect(ctx, req.Url, req.Headers, req.Body)
	// Convert to protobuf response
	techResponse := &pb.WappalyzerResponse{
		Technologies: make(map[string]*pb.Technologies),
	}

	for category, techs := range technologies {
		logger.Info().
			Str("request_id", requestID).
			Str("category", category).
			Strs("technologies", techs).
			Msg("category technologies")

		techResponse.Technologies[category] = &pb.Technologies{
			Names: techs,
		}
	}

	return &pb.Response{
		RequestId: requestID,  // Set the request ID in the response
		Success: true,
		Payload: &pb.Response_Wappalyzer{
			Wappalyzer: techResponse,
		},
	}, nil
}
```

Which in turn gets written back to our shared socket. Moving over the Rust side, the oppose it done.

## Rust

We have our container that handles all the internals of managing requests and responses.

```rust
type PendingMap = Arc<Mutex<BTreeMap<String, oneshot::Sender<Response>>>>;

/// HibsGopher service manager
#[derive(Debug, Clone)]
pub struct HibsGopher {
    config: HibsGopherConfig,

    /// Writer half of the Unix stream for sending requests
    writer: Arc<Mutex<OwnedWriteHalf>>,
    pending: PendingMap,
}
```

The most important being our `wappalyzer` request handler and our `recv` loop. Starting with our request creation.

```rust
pub async fn wappalyzer(
    &self,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
) -> Result<Response, HibsGopherError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let message = Request {
        request_id,
        r#type: RequestType::Wappalyzer.into(),
        payload: Some(Payload::Wappalyzer(WappalyzerRequest {
            url: url.clone(),
            headers,
            body,
        })),
    };

    let (tx, rx) = oneshot::channel();

    // Personal preference is to keep lock scopes quite explicit
    {
        let mut pending = self.pending.lock().await;
        pending.insert(message.request_id.to_string(), tx);
    }

    {
        let mut stream = self.writer.lock().await;
        let encoded_message = message.encode_length_delimited_to_vec();
        tracing::debug!(
            "attempting to send message to hibs-gopher: {} bytes",
            encoded_message.len()
        );
        stream
            .write_all(encoded_message.as_slice())
            .await
            .map_err(HibsGopherError::SocketError)?;
        tracing::debug!("message sent to hibs-gopher");
    }

    rx.await.map_err(|_| HibsGopherError::ResponseChannelClosed)
}
```

The part I liked most about this is, is that this will handle waiting for the response, and parses it back to our internal type. This means that downstream crates that use this library do not have to worry much about the internals (besides handling errors that is). Our `recv` listens for a message, and pulls the necessary number of bytes based on the variable integer size. We went with varint here as our message sizes vary quite a bit, and the upperbound is very high, albeit infrequently so.

```rust
pub async fn recv(
    pending: PendingMap,
    mut reader: OwnedReadHalf,
) -> Result<(), HibsGopherError> {
    loop {
        // Read the varint-encoded length prefix.
        let message_len = match Self::read_varint(&mut reader).await {
            Ok(len) => len as usize,
            Err(e) => {
                tracing::error!("error reading varint length from socket: {e:?}");
                continue;
            }
        };

        // Allocate a buffer for the full message.
        let mut message = vec![0u8; message_len];
        if let Err(e) = reader.read_exact(&mut message).await {
            tracing::error!("error reading message body: {e:?}");
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Attempt to decode the response.
        if let Ok(response) = proto::Response::decode(&message[..]) {
            let request_id = response.request_id.clone();
            let mut pending = pending.lock().await;
            if let Some(tx) = pending.remove(&request_id) {
                let _ = tx.send(response);
            } else {
                tracing::warn!("no pending requests found for response {request_id}");
            }
        } else {
            tracing::error!("unable to parse bytes into `proto::Response`, bytes: {message:?}");
            continue;
        }
    }
}
```

# Improving ergonomics

Start documenting how we wrapped it around a Rust library crate

# Caveats

Challenges with lambda?

[^1]: https://news.ycombinator.com/item?id=37236746
