use std::collections::VecDeque;
use bytes::{Bytes, BytesMut, BufMut, Buf};
use super::Message;
pub use tokio_util::codec::{Encoder, Decoder, Framed, FramedRead, FramedWrite, FramedParts};

/// Codec type for [`Message`] that implements [`tokio_util::codec::Decoder`] and [`tokio_util::codec::Encoder`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MessageCodec {
    frames: VecDeque<Bytes>,
    state: MessageCodecState,
}

impl MessageCodec {
    const NO_FRAME: u8 = 0;
    const HAS_FRAME: u8 = 1;

    pub fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.frames.clear();
        self.state = MessageCodecState::Pending;
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(item.byte_count());
        let frame_count = item.frames.len();
        item.frames.into_iter().enumerate().for_each(|(idx, frame)| {
            dst.put_u32(frame.len() as u32);
            dst.put_slice(&frame);

            if frame_count <= idx + 1 {
                // On the last frame, write the frame end byte.
                dst.put_u8(Self::NO_FRAME);
            } else {
                // else, it has a frame
                dst.put_u8(Self::HAS_FRAME);
            }
        });

        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        use std::io::ErrorKind;

        loop {
			match self.state {
				MessageCodecState::Pending => {
					if src.len() < 4 { return Ok(None) }
                    let frame_len = src.get_u32() as usize;
					src.reserve(frame_len + 4 + 1);
					self.state = MessageCodecState::AfterLength { frame_len };
					
					if src.len() < frame_len + 1 { return Ok(None) }
                    let frame = src.copy_to_bytes(frame_len);
					let frame_end_byte = src.get_u8();
					self.frames.push_back(frame);
					self.state = MessageCodecState::Pending;
		
					match frame_end_byte {
						Self::NO_FRAME => break,
						Self::HAS_FRAME => continue,
						_ => return Err(std::io::Error::new(ErrorKind::InvalidInput, "unrecognized frame end byte"))
					}
				},
				MessageCodecState::AfterLength { frame_len } => {
					if src.len() < frame_len + 1 { return Ok(None) }
                    let frame = src.copy_to_bytes(frame_len);
					let frame_end_byte = src.get_u8();
					self.frames.push_back(frame);
					self.state = MessageCodecState::Pending;
		
					match frame_end_byte {
						Self::NO_FRAME => break,
						Self::HAS_FRAME => continue,
						_ => return Err(std::io::Error::new(ErrorKind::InvalidInput, "unrecognized frame end byte"))
					}
				},
			}
        }

        let frames = self.frames.drain(..).collect();
        self.clear();
        Ok(Some(Self::Item {
            frames,
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MessageCodecState {
    Pending,
    AfterLength { frame_len: usize },
}

impl Default for MessageCodecState {
    fn default() -> Self {
        Self::Pending
    }
}
