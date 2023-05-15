# wire-framed

wire-framed is a library for encoding and decoding messages using a custom binary protocol.
It prioritizes ease of use and performance.

It revolves around the `Message` type that can hold multiple frames of data (represented with the `bytes::Bytes` type), 
and 4 main traits: `FromFrame`, `IntoFrame`, `FromMessage` and `IntoMessage`.

Each frame should be a self-contained piece of data that can be decoded without any context.

# Usage with frames
```rust
use wire_framed::prelude::*;

#[derive(Debug, Encoding, Decoding, PartialEq, Eq)]
pub struct Foo {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub created_at: u64,
}

fn send() -> Result<(), std::io::Error> {
    let foo = Foo {
        id: 1,
        name: "John".to_string(),
        description: "John is a legend".to_string(),
        created_at: 1234567890,
    };

    let frame = foo.into_frame();
    send_to_socket(frame)
}

fn recv() -> Result<(), std::io::Error> {
    let bytes = recv_from_socket();
    let foo = Foo::from_frame(bytes)?;
    
    // process foo
}
```

# Usage with messages
```rust
use wire_framed::prelude::*;

#[derive(Debug, Encoding, Decoding, PartialEq, Eq)]
pub struct Foo {
    pub id: u32,
    pub name: String,
    pub description: String,
}

fn main() -> Result<(), std::io::Error> {
    let foo = Foo {
         id: 1,
         name: "John".to_string(),
         description: "John is a legend".to_string(),
    };

    let msg = Message::builder()
        .frame(foo.into_frame())
        .frame(foo.into_frame())
        .build();
    
    println!("Message: {:?}", msg);
    Ok(())
}
