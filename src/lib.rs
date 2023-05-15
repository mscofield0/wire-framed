//! wire-framed is a library for encoding and decoding messages using a custom binary protocol.
//! It prioritizes ease of use and performance.
//! 
//! It revolves around the [`Message`] type that can hold multiple frames of data (represented with the [`bytes::Bytes`] type), 
//! and 4 main traits: [`FromFrame`], [`IntoFrame`], [`FromMessage`] and [`IntoMessage`].
//! 
//! Each frame should be a self-contained piece of data that can be decoded without any context.
//! 
//! # Usage with frames
//! ```
//! use wire_framed::prelude::*;
//! 
//! #[derive(Debug, Encoding, Decoding, PartialEq, Eq)]
//! pub struct Foo {
//!     pub id: u32,
//!     pub name: String,
//!     pub description: String,
//!     pub created_at: u64,
//! }
//! 
//! # fn send_to_socket(_frame: Bytes) -> Result<(), std::io::Error> { Ok(()) }
//! 
//! fn send() -> Result<(), std::io::Error> {
//!     let foo = Foo {
//!         id: 1,
//!         name: "John".to_string(),
//!         description: "John is a legend".to_string(),
//!         created_at: 1234567890,
//!     };
//! 
//!     let frame = foo.into_frame();
//!     send_to_socket(frame)
//! }
//! 
//! # fn recv_from_socket() -> Bytes {
//! #     Bytes::from_static(&[
//! #        0x00, 0x00, 0x00, 0x01, // id
//! #        0x00, 0x00, 0x00, 0x04, // name length
//! #        0x4a, 0x6f, 0x68, 0x6e, // name
//! #        0x00, 0x00, 0x00, 0x10, // description length
//! #        0x4a, 0x6f, 0x68, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x65, 0x67, 0x65, 0x6e, 0x64, // description
//! #        0x00, 0x00, 0x00, 0x00, 0x49, 0x96, 0x02, 0xd2, // created_at
//! #     ])
//! # }
//! 
//! fn recv() -> Result<(), std::io::Error> {
//!     let bytes = recv_from_socket();
//!     let foo = Foo::from_frame(bytes)?;
//!
//!     // process foo
//! #   Ok(())
//! }
//! 
//! # fn main() -> Result<(), std::io::Error> {
//! #     send()?;
//! #     recv()?;
//! #
//! #     Ok(())
//! # }
//! ```
//! 
//! # Usage with messages
//! ```
//! use wire_framed::prelude::*;
//! 
//! #[derive(Debug, Encoding, Decoding, PartialEq, Eq)]
//! pub struct Foo {
//!     pub id: u32,
//!     pub name: String,
//!     pub description: String,
//! }
//! 
//! fn main() -> Result<(), std::io::Error> {
//!     let foo = Foo {
//!          id: 1,
//!          name: "John".to_string(),
//!          description: "John is a legend".to_string(),
//!     };
//! 
//!     let msg = Message::builder()
//!         .frame(foo.into_frame())
//!         .frame(foo.into_frame())
//!         .build();
//!     
//!     println!("Message: {:?}", msg);
//!     Ok(())
//! }


pub use wire_framed_core::{
    self, FromFrame, FromMessage, IntoFrame, IntoMessage, Message, 
    MessageCodec, MessageBuilder, bytes::{self, Bytes, BytesMut, Buf, BufMut}, codec, common_impls::*, utils
};
pub use wire_framed_derive::{Decoding, Encoding};

pub mod prelude {
    pub use super::*;
}