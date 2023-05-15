use std::hash::Hash;
use std::collections::HashSet;
use bytes::BufMut;

use super::*;

impl FromFrame for bool {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_bool(frame, "bool")
	}
}

impl FromFrame for u8 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_u8(frame, "u8")
	}
}

impl FromFrame for u16 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_u16(frame, "u16")
	}
}

impl FromFrame for u32 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_u32(frame, "u32")
	}
}

impl FromFrame for u64 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_u64(frame, "u64")
	}
}

impl FromFrame for i8 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_i8(frame, "i8")
	}
}

impl FromFrame for i16 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_i16(frame, "i16")
	}
}

impl FromFrame for i32 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_i32(frame, "i32")
	}
}

impl FromFrame for i64 {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_i64(frame, "i64")
	}
}


impl FromFrame for String {
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_string(frame, "string")
	}
}

impl<T> FromFrame for Option<T>
where
	T: FromFrame,
	<T as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_option(frame, "option", |frame| <T as FromFrame>::parse_frame(frame).map_err(Into::into))
	}
}

impl<T> FromFrame for Vec<T>
where
	T: FromFrame,
	<T as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_array(frame, "array", |frame| <T as FromFrame>::parse_frame(frame).map_err(Into::into))
	}
}

impl<T> FromFrame for HashSet<T>
where
	T: FromFrame + PartialEq + Eq + Hash,
	<T as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		utils::get_hashset(frame, "hashset", |frame| <T as FromFrame>::parse_frame(frame).map_err(Into::into))
	}
}

impl IntoFrame for bool {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u8(*self as u8)
	}
}

impl IntoFrame for u8 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u8(*self)
	}
}

impl IntoFrame for u16 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u16(*self)
	}
}

impl IntoFrame for u32 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u32(*self)
	}
}

impl IntoFrame for u64 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u64(*self)
	}
}

impl IntoFrame for i8 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_i8(*self)
	}
}

impl IntoFrame for i16 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_i16(*self)
	}
}

impl IntoFrame for i32 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_i32(*self)
	}
}

impl IntoFrame for i64 {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_i64(*self)
	}
}

impl IntoFrame for &str {
	fn extend_frame(&self, frame: &mut BytesMut) {
		frame.put_u32(self.len() as u32);
		frame.put_slice(self.as_bytes());
	}

	fn size_hint(&self) -> usize {
		4 + self.len()
	}
}

impl IntoFrame for String {
	fn extend_frame(&self, frame: &mut BytesMut) {
		<&str as IntoFrame>::extend_frame(&self.as_str(), frame)
	}

	fn size_hint(&self) -> usize {
		<&str as IntoFrame>::size_hint(&self.as_str())
	}
}

impl<T: IntoFrame> IntoFrame for Option<T> {
	fn extend_frame(&self, frame: &mut BytesMut) {
		utils::put_option(frame, self, |frame, value| <T as IntoFrame>::extend_frame(value, frame));
	}

	fn size_hint(&self) -> usize {
		1 + self.as_ref().map(|value| value.size_hint()).unwrap_or(0)
	}
}

impl<T: IntoFrame> IntoFrame for Vec<T> {
	fn extend_frame(&self, frame: &mut BytesMut) {
		utils::put_array(frame, self, |frame, value| <T as IntoFrame>::extend_frame(value, frame));
	}

	fn size_hint(&self) -> usize {
		4 + self.iter().map(|value| value.size_hint()).sum::<usize>()
	}
}

impl<T: IntoFrame + PartialEq + Eq + Hash> IntoFrame for HashSet<T> {
	fn extend_frame(&self, frame: &mut BytesMut) {
		utils::put_hashset(frame, self, |frame, value| <T as IntoFrame>::extend_frame(value, frame));
	}

	fn size_hint(&self) -> usize {
		4 + self.iter().map(|value| value.size_hint()).sum::<usize>()
	}
}
