//! wire-framed is a library for encoding and decoding frames using a custom binary protocol.
//! It prioritizes ease-of-use.
//! 
//! It reolves around two traits [`FromFrame`] and [`IntoFrame`]. These traits can be manually implemented relatively easily using 
//! the utilities provided in the [`utils`] module or automatically using the [`Encoding`] and [`Decoding`] macros.
//! 
//! [`FromFrame`]: trait.FromFrame.html
//! [`IntoFrame`]: trait.IntoFrame.html
//! [`utils`]: utils/index.html
//! [`Encoding`]: macro.Encoding.html
//! [`Decoding`]: macro.Decoding.html
//! 
//! # Usage
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


pub use wire_framed_core::{
    self, FromFrame, IntoFrame, FrameCodec, bytes::{self, Bytes, BytesMut, Buf, BufMut}, codec, common_impls::*, utils
};
pub use wire_framed_derive::{Decoding, Encoding};

pub mod prelude {
    pub use super::*;
}