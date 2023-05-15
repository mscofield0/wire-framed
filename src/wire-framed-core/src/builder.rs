use std::collections::VecDeque;
use bytes::Bytes;
use crate::Message;

/// Builder type for [`Message`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBuilder {
	frames: VecDeque<Bytes>,
}

impl MessageBuilder {
	/// Construct empty [`MessageBuilder`].
	pub fn new() -> Self {
        Self {
            frames: Default::default(),
        }
    }

	/// Push a frame to the [`Message`].
	pub fn frame(mut self, frame: Bytes) -> Self {
		self.frames.push_back(frame);
		self
	}

	/// Extend the current [`Message`] with another [`Message`].
	pub fn message(mut self, mut msg: Message) -> Self {
		self.frames.append(&mut msg.frames);
		self
	}

	/// Finalize and build the [`Message`].
	pub fn build(self) -> Message {
        Message {
            frames: self.frames,
        }
    }
}
