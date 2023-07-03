use bytes::{Bytes, BytesMut, BufMut, Buf};
pub use tokio_util::codec::{Decoder, Encoder};

pub type Framed<S> = tokio_util::codec::Framed<S, FrameCodec>;
pub type FramedRead<S> = tokio_util::codec::FramedRead<S, FrameCodec>;
pub type FramedWrite<S> = tokio_util::codec::FramedWrite<S, FrameCodec>;

/// Codec type for [`Message`] that implements [`tokio_util::codec::Decoder`] and [`tokio_util::codec::Encoder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameCodec {
    byte_count: Option<u32>,
    data: BytesMut,
}

impl FrameCodec {
    pub fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.byte_count = None;
        self.data.clear();
    }
}

impl Default for FrameCodec {
    fn default() -> Self {
        Self {
            byte_count: None,
            data: BytesMut::new(),
        }
    }
}

impl Encoder<Bytes> for FrameCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let byte_count = item.len() as u32;
        dst.reserve(std::mem::size_of::<u32>() + byte_count as usize);
        dst.put_u32(byte_count);
        dst.put(item);

        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // read the initial frame length
        if self.byte_count.is_none() {
            if src.len() < std::mem::size_of::<u32>() {
                return Ok(None);
            }

            let byte_count = src.get_u32();
            self.data.reserve(byte_count as usize);
            self.byte_count = Some(byte_count);
        }

        // read chunk of data
        let byte_count = self.byte_count.unwrap();
        let remaining_bytes = (byte_count - self.data.len() as u32) as usize;
        let at = std::cmp::min(remaining_bytes, src.len());
        self.data.put(src.split_to(at));

        // if we have read all the data, return the frame
        if byte_count == self.data.len() as u32 {
            let frame = self.data.clone().freeze();
            self.clear();
            return Ok(Some(frame))
        }

        // otherwise, wait for more data to arrive to finish the frame
        Ok(None)
    }
}
