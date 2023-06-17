---
title: Implementing an Event Bus using Rust
date: 06/03/2023
description: An introductory post that shows how I've gone about implementing a rudimentary broadcast event bus using tokio in order to build a module system with message passing.
---
## Introduction

Lately I have been looking to learn and experiment more with lower level systems engineering projects. One of these ideas was to build a nifty little product, similar to the [FlipperZero](https://flipperzero.one/) that acts as a fun, educational, offensive WiFi security device &mdash; the initial plan being to base it off of the Raspberry Pi 0W and combining eBPF to have a very low footprint. 

Before digging into the network aspect of the device I decided to begin with higher level requirements and build a small framework that will enable an effective development and implementation of the device. A key requirement is the ability for different parts of the device to operate asynchronously, such as UI updates, hardware inputs, listening to network traffic as well as auxiliary functions such as logging. We need to allow for modules to operate independently while also being able to listen to what other modules are doing.

One common pattern that enables effective communication between these different modules is the event bus pattern. This post aims to introduce the pattern briefly, with a concrete example[^1], and how it enables this particular use-case. We will be implementing an example event bus in Rust using the `tokio` runtime, focusing on a use case where different modules within our system need to communicate with each other.

## Event Bus Pattern

> **Note** &mdash; I use channels and bus interchangeably. While there is certainly overlap, they are not exatly the same. Channels refer to the low-level construct used for thread or task communication which is what we are doing. The event bus pattern however can be applied at multiple layers with increasing degrees of abstraction.

The event bus pattern provides a mechanism for different parts of a system to communicate with each other without needing to be directly connected. Modules send events to the bus and listen for events from the bus. This decouples senders and receivers of events, allowing for more flexible and scalable systems. 

<!-- Insert diagram -->

In our example we'll be focusing on a broadcasting event bus&mdash;somewhat different to a publish-subscribe pattern. In our example all actors receive messages on the bus and it is up to each actor to know which messages to ignore and which to process.

## Implementing an Event Bus with Tokio

The `tokio` runtime in Rust provides the tools necessary to implement an event bus pattern. In particular, `tokio` provides a broadcast channel that is well-suited to this task. A broadcast channel allows a single sender to notify multiple receivers, which aligns perfectly with our need for a single event bus to communicate with multiple modules.

We implemented our `EventBus` as a wrapper around a `tokio::sync::broadcast` channel. The `EventBus` allows any module to clone it and send events, effectively acting as a publisher. On the other hand, any module can also listen for events from the `EventBus`, acting as a subscriber.

Here is a simplified version of our `EventBus` implementation:

```rust
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct Event {
    pub module: String,
    pub inner: EventKind,
}

struct EventBus {
    sender: broadcast::Sender<Event>,
}

#[derive(Clone, Debug)]
pub enum EventKind {
    StubEvent(String),
}

impl EventBus {
    fn new() -> Self {
        let (sender, _) = broadcast::channel(100);

        Self { sender }
    }

    fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    fn publish(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
```

`Event` is a type that represents the different kinds of events that can be sent through the event bus. The `new` function creates a new `EventBus` with a broadcast channel that can hold up to 100 events. The `subscribe` function allows a module to receive a receiver for the broadcast channel, enabling it to listen for events. The `publish` function allows a module to send an event to the event bus. `EventKind` is a simple enum that allows you to encode multiple different events; ideally each module owns its own event type.

## Introducing the concept of Modules

With the basic event bus ready, we turn over to implementing the concept of a `Module`. Each module encapsulates some unit of work that operates independently of other modules. 

```rust
use crate::event_bus::{Event, EventBus};

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait Module {
    fn new(ctx: ModuleCtx) -> Self;
    async fn run(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct ModuleCtx {
    pub sender: broadcast::Sender<Event>,
    pub receiver: broadcast::Receiver<Event>,
}


impl ModuleCtx {
    pub fn new(bus: &EventBus) -> Self {
        let sender = bus.sender.clone();
        let receiver = bus.subscribe();

        ModuleCtx { sender, receiver }
    }
}
```

`ModuleCtx` is a container holding all the metadata needed by the module to start and run. Initialising a module creates a new sender to the bus and receiver subscribing to events. We add a `Module` trait, which is arguable, however it ensures that every module can be `run()` and running is fallible.

## Minimal example

With modules, events and the event bus defined, we can start implementing a few basic modules. In our example we'll have two modules:

* A network module that listens to all network traffic receied over WiFi and emits it to the bus.
* An auxiliary logging module that simply listens to all events on the bus and neatly logs them.

#### Network module

> Initially this example was meant to use [`libpnet`](https://docs.rs/pnet/latest/pnet/) to create a listener. It was substantially longer and deviated from the purpose of the post.

```rust
use crate::event_bus::{Event, EventKind};
use crate::module::{Module, ModuleCtx};
use anyhow::Result;
use async_trait::async_trait;

pub struct Network {
    pub name: String,
    ctx: ModuleCtx,
}

#[async_trait]
impl Module for Network {
    fn new(ctx: ModuleCtx) -> Self {
        Worker {
            name: String::from("network"),
            ctx,
        }
    }

    async fn run(&mut self) -> Result<()> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let event = Event {
                        module: self.name.to_string(),
                        inner: EventKind::StubEvent("Received some packet".to_string()),
                    };
                    self.ctx.sender
                        .send(event)
                        .unwrap();
                },
            }
        }
    }

}
```

#### Wrapping up

On startup, we create the event bus, initialise the modules and run them.

```rust
use anyhow::Result;
use event_bus::EventBus;
use logger_module::Logger;
use module::{Module, ModuleCtx};
use worker_module::Worker;

#[tokio::main]
async fn main() -> Result<()> {
    let event_bus = EventBus::new();


    let logger_ctx = ModuleCtx::new(&event_bus);
    let mut logger = Logger::new(logger_ctx);

    let worker_ctx = ModuleCtx::new(&event_bus);
    let mut worker = Worker::new(worker_ctx);

    tokio::join!(worker.run(), logger.run()).0?;

    Ok(())
}
```

#### Logger module

```rust
use crate::event_bus::EventKind;
use crate::module::{Module, ModuleCtx};
use anyhow::Result;
use async_trait::async_trait;

pub struct Logger {
    pub name: String,
    ctx: ModuleCtx,
}

#[async_trait]
impl Module for Logger {
    fn new(ctx: ModuleCtx) -> Self {
        Logger {
            name: String::from("logger"),
            ctx,
        }
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                e = self.ctx.receiver.recv() => {
                    match e {
                        Ok(event) => {
                            match event.inner {
                                EventKind::StubEvent(message) => println!("{}: received event: {}", &self.name, message),
                            }
                        },
                        Err(e) => println!("Error: {}", e),
                    }

                },
            }
        }

    }
}
```


## Conclusion

The event bus pattern provides a powerful mechanism for communication in a modular system. With Rust's `tokio` runtime, we can implement an event bus that allows for asynchronous, non-blocking communication between different modules.

However, there's still more to explore. The current implementation can be enhanced further by adding more sophisticated event types, better error handling, and perhaps even integrating with other asynchronous libraries and frameworks. This exploration forms the foundation for building more complex, robust and flexible systems in Rust.

[^1]: https://github.com/JuxhinDB/event-bus-example/blob/main/src/main.rs
