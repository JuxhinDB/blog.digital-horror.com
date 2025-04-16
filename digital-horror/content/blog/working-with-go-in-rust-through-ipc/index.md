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

You would effectively need to export functions using `cgo` and compile your code as a shared library (i.e., a C ABI). Rust, with its robust C FFI can then call these exported functions. Implementing this on the Go side, would look something like so.

```go
import "C"

//export Add
func Add(a, b C.int) C.int {
    return a + b
}
```

Besides the atrocious tagging of functions to be exported with comments, this is a viable option. I initially wanted to adopt this option as it was the most performant. It avoids de-/serialising data back and forth along with multiple memory copies and allocations. It's also not as complex as trying to get the Rust and Go binaries to share some common memory region.

In the end, I opted out of this option due to the added development friction it added. In short, it slowed things down too much, and was not as pleasant to work with.

## `rust2go`

The fact that we had to go about this indirectly really bothered me. I kept digging and came across the [`rust2go`](https://github.com/ihciah/rust2go) project which seemed impressive. Equally impressive is the magnificent [blog post](https://en.ihcblog.com/rust2go/) by Hai Chi, which I encourage anyone interested in this area to read a few times. Unfortunately while impressive, this flips the order of integration. What I was hoping to find, was something akin to `go2rust` instead, so this option was ruled out. However, the blog post did spark a simpler idea I hadn't thought of.

## Unix Domain Socket (UDS)

The simplest approach I found was to rely on inter-process communication (IPC) via unix sockets. Rather than having C ABI be our interface boundary, we can rely on some wire protocol (e.g., protobuf) to pass messages back and forth. This is a neat solution for us due to a number of reasons:

* Exceptionally simple and quick to implement
* Keeps boundaries separate
* Avoids memory safety issues
* Runs locally within the same lambda runtime context

The immediate and most obvious downside here is the performance. We need to serialise and deserialize large buffers (i.e., entire website DOMs) along with multiple kernel copies from our applications to the kernel. I mentioned earlier in this post that performance was not as critical. The reason being that when we need to make these calls, we're executing other external calls (i.e., inference, database queries) concurrently. Therefore in practice, the overhead does not translate to any meaningful final end-to-end overhead.

# Fleshing it out

Start documenting how we fleshed out the Go service and protobuf messages.

# Improving ergonomics

Start documenting how we wrapped it around a Rust library crate

# Caveats

Challenges with lambda?

[^1]: https://news.ycombinator.com/item?id=37236746
