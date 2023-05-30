pub mod codec;
pub mod utils;
pub mod common_impls;
pub use codec::{FrameCodec, Framed, FramedRead, FramedWrite};
pub use common_impls::*;
pub use bytes;

use bytes::{Bytes, BytesMut};

/// Trait for converting a frame into `Self.
pub trait FromFrame: Sized {
    /// The error type returned when parsing a frame.
    type Error;

    /// Parse a frame into a `Self`.
    fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error>;

    /// Convert a frame into a `Self`.
    fn from_frame(mut frame: Bytes) -> Result<Self, Self::Error> {
        Self::parse_frame(&mut frame)
    }
}

/// Trait for converting a `Self` into a frame.
pub trait IntoFrame: Sized {
    /// Extend a frame with the contents of `Self`.
    fn extend_frame(&self, frame: &mut BytesMut);

    /// Returns the size hint of `Self` in bytes.
    fn size_hint(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    /// Converts `Self` into an owned frame.
    fn into_frame(&self) -> Bytes {
        let mut frame = BytesMut::with_capacity(self.size_hint());
        self.extend_frame(&mut frame);
        frame.into()
    }
}


#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};
    use super::*;
    
    struct Test {
        id: u64,
        data: Data,
    }

    struct Data {
        a: u32,
        b: u32,
    }

    #[test]
    fn from_frame_test() {
        impl FromFrame for Test {
            type Error = anyhow::Error;

            fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
                let id = utils::get_u64(frame, "id")?;
                let data = Data {
                    a: utils::get_u32(frame, "data.a")?,
                    b: utils::get_u32(frame, "data.b")?,
                };
                
                Ok(Self {
                    id,
                    data,
                })
            }
        }

        let mut frame = BytesMut::new();
        frame.put_u64(42);
        frame.put_u32(127);
        frame.put_u32(72);

        let parsed = Test::from_frame(frame.into()).unwrap();
        assert_eq!(parsed.id, 42);
        assert_eq!(parsed.data.a, 127);
        assert_eq!(parsed.data.b, 72);
    }
    
    #[test]
    fn into_frame_test() {
        impl IntoFrame for Test {
            fn extend_frame(&self, frame: &mut BytesMut) {
                frame.put_u64(self.id);
                frame.put_u32(self.data.a);
                frame.put_u32(self.data.b);
            }
        }

        let test = Test { id: 42, data: Data { a: 127, b: 72 } };
        let result = test.into_frame();

        let target = Bytes::from_static(&[
            0, 0, 0, 0, 0, 0, 0, 42,
            0, 0, 0, 127,
            0, 0, 0, 72,
        ]);
        assert_eq!(&result, &target);
    }
}
