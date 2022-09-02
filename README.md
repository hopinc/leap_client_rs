# leap_edge_rs

Utility library for connecting and receiving events from [Leap Edge](https://docs.hop.io/docs/channels/internals/leap). Used for Channels and Pipe.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leap_edge_rs = "0.1"
```

or if you want to add features:

```toml
[dependencies.leap_edge_rs]
version = "0.1"
default-features = false
features = ["rustls-tls-webpki-roots", "zlib"]
```

## Usage

### Subscribe to a channel

```rust
use leap_edge_rs::{LeapEdge, LeapOptions, leap::types::Event};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let leap = LeapEdge::new(LeapOptions {
        project: "my-project",
        ..Default::default()
    }).await?;

    leap.channel_subscribe("my-channel").await?;

    while let Some(event) = leap.listen().await {
        println!("{:?}", event);
    }
}
```

### Get all events:

```rust
use leap_edge_rs::{LeapEdge, LeapOptions};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let leap = LeapEdge::new(LeapOptions {
        project: "my-project",
        ..Default::default()
    }).await?;

    while let Some(event) = leap.listen().await {
        println!("{:?}", event);
    }
}
```

### Get only messages or direct messages:

```rust
use leap_edge_rs::{LeapEdge, LeapOptions, leap::types::Event};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let leap = LeapEdge::new(LeapOptions {
        project: "my-project",
        ..Default::default()
    }).await?;

    while let Some(event) = leap.listen().await {
        match event {
            Event::Message(message) | Event::DirectMessage(message) => println!("{:?}", message),

            _ => {}
        }
    }
}
```