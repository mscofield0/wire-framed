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

impl IntoMessage for Message {
	fn extend_message(&self, msg: &mut Message) {
		self.frames.iter().for_each(|frame| msg.push(frame.clone()));
	}
}

impl IntoMessage for Bytes {
	fn extend_message(&self, msg: &mut Message) {
		msg.push(self.clone());
	}
}

impl<T1, T2> IntoFrame for (T1, T2)
where
	T1: IntoFrame,
	T2: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint()
	}
}

impl<T1, T2, T3> IntoFrame for (T1, T2, T3)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint()
	}
}

impl<T1, T2, T3, T4> IntoFrame for (T1, T2, T3, T4)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
	T4: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
		self.3.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint()
	}
}

impl<T1, T2, T3, T4, T5> IntoFrame for (T1, T2, T3, T4, T5)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
	T4: IntoFrame,
	T5: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
		self.3.extend_frame(frame);
		self.4.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint() + self.4.size_hint()
	}
}

impl<T1, T2, T3, T4, T5, T6> IntoFrame for (T1, T2, T3, T4, T5, T6)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
	T4: IntoFrame,
	T5: IntoFrame,
	T6: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
		self.3.extend_frame(frame);
		self.4.extend_frame(frame);
		self.5.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint() + self.4.size_hint() + self.5.size_hint()
	}
}

impl<T1, T2, T3, T4, T5, T6, T7> IntoFrame for (T1, T2, T3, T4, T5, T6, T7)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
	T4: IntoFrame,
	T5: IntoFrame,
	T6: IntoFrame,
	T7: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
		self.3.extend_frame(frame);
		self.4.extend_frame(frame);
		self.5.extend_frame(frame);
		self.6.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint() + self.4.size_hint() + self.5.size_hint() + self.6.size_hint()
	}
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> IntoFrame for (T1, T2, T3, T4, T5, T6, T7, T8)
where
	T1: IntoFrame,
	T2: IntoFrame,
	T3: IntoFrame,
	T4: IntoFrame,
	T5: IntoFrame,
	T6: IntoFrame,
	T7: IntoFrame,
	T8: IntoFrame,
{
	fn extend_frame(&self, frame: &mut BytesMut) {
		self.0.extend_frame(frame);
		self.1.extend_frame(frame);
		self.2.extend_frame(frame);
		self.3.extend_frame(frame);
		self.4.extend_frame(frame);
		self.5.extend_frame(frame);
		self.6.extend_frame(frame);
		self.7.extend_frame(frame);
	}

	fn size_hint(&self) -> usize {
		self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint() + self.4.size_hint() + self.5.size_hint() + self.6.size_hint() + self.7.size_hint()
	}
}

impl<T1, T2> FromFrame for (T1, T2)
where
	T1: FromFrame,
	T2: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3> FromFrame for (T1, T2, T3)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3, T4> FromFrame for (T1, T2, T3, T4)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	T4: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
	<T4 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?, T4::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3, T4, T5> FromFrame for (T1, T2, T3, T4, T5)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	T4: FromFrame,
	T5: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
	<T4 as FromFrame>::Error: Into<std::io::Error>,
	<T5 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?, T4::parse_frame(frame).map_err(Into::into)?, T5::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3, T4, T5, T6> FromFrame for (T1, T2, T3, T4, T5, T6)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	T4: FromFrame,
	T5: FromFrame,
	T6: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
	<T4 as FromFrame>::Error: Into<std::io::Error>,
	<T5 as FromFrame>::Error: Into<std::io::Error>,
	<T6 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?, T4::parse_frame(frame).map_err(Into::into)?, T5::parse_frame(frame).map_err(Into::into)?, T6::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3, T4, T5, T6, T7> FromFrame for (T1, T2, T3, T4, T5, T6, T7)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	T4: FromFrame,
	T5: FromFrame,
	T6: FromFrame,
	T7: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
	<T4 as FromFrame>::Error: Into<std::io::Error>,
	<T5 as FromFrame>::Error: Into<std::io::Error>,
	<T6 as FromFrame>::Error: Into<std::io::Error>,
	<T7 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?, T4::parse_frame(frame).map_err(Into::into)?, T5::parse_frame(frame).map_err(Into::into)?, T6::parse_frame(frame).map_err(Into::into)?, T7::parse_frame(frame).map_err(Into::into)?))
	}
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> FromFrame for (T1, T2, T3, T4, T5, T6, T7, T8)
where
	T1: FromFrame,
	T2: FromFrame,
	T3: FromFrame,
	T4: FromFrame,
	T5: FromFrame,
	T6: FromFrame,
	T7: FromFrame,
	T8: FromFrame,
	<T1 as FromFrame>::Error: Into<std::io::Error>,
	<T2 as FromFrame>::Error: Into<std::io::Error>,
	<T3 as FromFrame>::Error: Into<std::io::Error>,
	<T4 as FromFrame>::Error: Into<std::io::Error>,
	<T5 as FromFrame>::Error: Into<std::io::Error>,
	<T6 as FromFrame>::Error: Into<std::io::Error>,
	<T7 as FromFrame>::Error: Into<std::io::Error>,
	<T8 as FromFrame>::Error: Into<std::io::Error>,
{
	type Error = std::io::Error;

	fn parse_frame(frame: &mut Bytes) -> Result<Self, Self::Error> {
		Ok((T1::parse_frame(frame).map_err(Into::into)?, T2::parse_frame(frame).map_err(Into::into)?, T3::parse_frame(frame).map_err(Into::into)?, T4::parse_frame(frame).map_err(Into::into)?, T5::parse_frame(frame).map_err(Into::into)?, T6::parse_frame(frame).map_err(Into::into)?, T7::parse_frame(frame).map_err(Into::into)?, T8::parse_frame(frame).map_err(Into::into)?))
	}
}
