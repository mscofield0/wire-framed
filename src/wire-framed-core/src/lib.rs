pub mod codec;
mod builder;
pub mod utils;
pub mod common_impls;
pub use builder::MessageBuilder;
pub use codec::MessageCodec;
pub use common_impls::*;
pub use bytes;

use std::collections::VecDeque;
use bytes::{Bytes, BytesMut};
use tokio_util::codec::Encoder;

/// A message with frames.
/// 
/// # Frame
/// A frame consists of the length of the blob and the blob itself.
/// 
/// | blob length | blob |
/// 
/// # Message
/// A message consists of (potentially) many frames. After a frame, a special byte 
/// exists to denote if the message has more frames to read. The byte contains 
/// 0 if it has no more frames and 1 if it has.
/// 
/// | frame | frame end byte |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    frames: VecDeque<Bytes>,
}

impl Message {
    /// Construct an empty [`Message`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a [`Message`] from an iterator of [`Frame`]s.
    pub fn from_frames(frames: impl Into<VecDeque<Bytes>>) -> Self {
        Self {
            frames: frames.into(),
        }
    }

    /// Constructs a [`MessageBuilder`].
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }

    /// Push a frame into the message.
    pub fn push(&mut self, frame: Bytes) {
        self.frames.push_back(frame);
    }

    /// Extends a collection with the contents of an iterator.
    pub fn extend(&mut self, frames: impl IntoIterator<Item = Bytes>) {
        self.frames.extend(frames);
    }
    
    /// Push a message into the message.
    pub fn push_message(&mut self, msg: Message) {
        self.frames.extend(msg.frames.into_iter());
    }

    /// Prepend a frame into the message.
    pub fn prepend(&mut self, frame: Bytes) {
        self.frames.push_front(frame);
    }
    
    /// Pop a frame off the message.
    pub fn pop(&mut self) -> Option<Bytes> {
        self.frames.pop_front()
    }

    /// Returns the number of frames in the message.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns the total length of the message in bytes.
    pub fn byte_count(&self) -> usize {
        // Frame size + frame data + frame end byte
        self.frames.iter().map(|frame| frame.len() + 4 + 1).sum()
    }

    /// Returns the message as an immutable [`VecDeque`] of frames.
    pub fn as_deque(&self) -> &VecDeque<Bytes> {
        &self.frames
    }

    /// Returns the message as a mutable [`VecDeque`] of frames.
    pub fn as_deque_mut(&mut self) -> &mut VecDeque<Bytes> {
        &mut self.frames
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            frames: Default::default(),
        }
    }
}

/// Trait for converting a [`Message`] to a `Self`.
pub trait FromMessage: Sized {
    /// The error type returned when parsing a [`Message`].
    type Error;

    /// Parse (while consuming) a [`Message`] into a `Self`.
    fn parse_message(msg: &mut Message) -> Result<Self, Self::Error>;

    /// Convert an owned [`Message`] into a `Self`.
    fn from_message(mut msg: Message) -> Result<Self, Self::Error> {
        Self::parse_message(&mut msg)
    }
}

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

/// Trait for converting a `Self` into a [`Message`].
pub trait IntoMessage: Sized {
    /// Extend a [`Message`] with the contents of `Self`.
    fn extend_message(&self, msg: &mut Message);

    /// Converts `Self` into an owned [`Message`].
    fn into_message(&self) -> Message {
        let mut msg = Message::new();
        self.extend_message(&mut msg);
        msg
    }

    fn into_bytes(&self) -> Bytes {
        let msg = self.into_message();
        let mut codec = MessageCodec::default();
        let mut bytes = BytesMut::with_capacity(msg.byte_count());
        codec.encode(msg, &mut bytes).expect("a message should never fail to be encoded");
        bytes.into()
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
    use std::mem::size_of;
    use bytes::{BufMut, BytesMut, Buf};
    use rassert_rs::rassert;
    use tokio_util::codec::{Decoder, Encoder};
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
    fn from_message_test() {
        impl FromMessage for Test {
            type Error = anyhow::Error;

            fn parse_message(msg: &mut Message) -> Result<Self, Self::Error> {
                let mut id_frame = msg.pop().ok_or(anyhow::anyhow!("missing id frame"))?;
                let mut data_frame = msg.pop().ok_or(anyhow::anyhow!("missing data frame"))?;

                rassert!(id_frame.len() == size_of::<u64>() as usize, anyhow::anyhow!("id frame length mismatch, length: {}", id_frame.len()));
                let id = id_frame.get_u64();

                rassert!(data_frame.len() == size_of::<Data>() as usize, anyhow::anyhow!("data frame length mismatch, length: {}", data_frame.len()));
                let data = Data {
                    a: data_frame.get_u32(),
                    b: data_frame.get_u32(),
                };
                
                Ok(Self {
                    id,
                    data,
                })
            }
        }

        let mut msg = Message::default();
        msg.push(Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]));
        msg.push(Bytes::from_static(&[0, 0, 0, 127, 0, 0, 0, 72]));
        let parsed = Test::from_message(msg).unwrap();
        assert_eq!(parsed.id, 42);
        assert_eq!(parsed.data.a, 127);
        assert_eq!(parsed.data.b, 72);
    }
    
    #[test]
    fn into_message_test() {
        impl IntoMessage for Test {
            fn extend_message(&self, msg: &mut Message) {
                let mut id_frame = BytesMut::new();
                id_frame.put_u64(self.id);

                let mut data_frame = BytesMut::new();
                data_frame.put_u32(self.data.a);
                data_frame.put_u32(self.data.b);

                msg.push(id_frame.into());
                msg.push(data_frame.into());
            }
        }

        let test = Test { id: 42, data: Data { a: 127, b: 72 } };
        let msg = test.into_message();

        let id_frame = Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]);
        let data_frame = Bytes::from_static(&[0, 0, 0, 127, 0, 0, 0, 72]);
        let result = Message::from_frames([
            id_frame,
            data_frame,
        ]);
        assert_eq!(&result, &msg);
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

    #[test]
    fn encode_test() {
        let mut codec = MessageCodec::default();
        let msg = Message::builder()
            .frame(Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]))
            .frame(Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]))
            .frame(Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]))
            .build();

        let mut dst = BytesMut::with_capacity(1024);
        codec.encode(msg, &mut dst).unwrap();

        //  length        data                       frame end byte
        let result: &[u8] = &[
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   1,
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   1,
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   0,
        ];
        assert_eq!(&result, &Vec::from(dst));
    }

    #[test]
    fn decode_test() {
        let mut src: BytesMut = BytesMut::new();
        src.extend_from_slice(&[
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   1,
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   1,
            0, 0, 0, 8,   0, 0, 0, 0, 0, 0, 0, 42,   0,
        ]);

        let mut codec = MessageCodec::default();
        let msg = codec.decode(&mut src).unwrap().unwrap();

        let frame = Bytes::from_static(&[0, 0, 0, 0, 0, 0, 0, 42]);
        let result = Message::from_frames([
            frame.clone(),
            frame.clone(),
            frame.clone(),
        ]);
        assert_eq!(&result, &msg);
    }

    #[test]
    fn empty_msg_test() {
        let src = Message::default();
        let mut dst = BytesMut::default();
        let mut codec = MessageCodec::default();
        codec.encode(src, &mut dst).unwrap();
        assert_eq!(dst.len(), 0);

        let mut src = BytesMut::default();
        let mut codec = MessageCodec::default();
        let dst = codec.decode_eof(&mut src).unwrap();
        assert_eq!(dst, None);
    }
}
