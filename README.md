# wire-framed

wire-framed is a library for encoding and decoding frames using a custom binary protocol.
It prioritizes ease-of-use.

It reolves around two traits [`FromFrame`] and [`IntoFrame`]. These traits can be manually implemented relatively easily using 
the utilities provided in the [`utils`] module or automatically using the [`Encoding`] and [`Decoding`] macros.

[`FromFrame`]: trait.FromFrame.html
[`IntoFrame`]: trait.IntoFrame.html
[`utils`]: utils/index.html
[`Encoding`]: macro.Encoding.html
[`Decoding`]: macro.Decoding.html

# Usage
```
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
