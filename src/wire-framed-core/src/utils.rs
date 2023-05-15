use bytes::{Buf, Bytes, BufMut, BytesMut};
use std::{io::{self, ErrorKind}, collections::HashSet, hash::Hash};

/// A utility function to get a [`bool`] from a [`Bytes`].
pub fn get_bool(src: &mut Bytes, name: &str) -> Result<bool, std::io::Error> {
	if src.len() < 1 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_u8() != 0)
}

/// A utility function to get a [`u8`] from a [`Bytes`].
pub fn get_u8(src: &mut Bytes, name: &str) -> Result<u8, std::io::Error> {
	if src.len() < 1 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_u8())
}

/// A utility function to get a [`u16`] from a [`Bytes`].
pub fn get_u16(src: &mut Bytes, name: &str) -> Result<u16, std::io::Error> {
	if src.len() < 2 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_u16())
}

/// A utility function to get a [`u32`] from a [`Bytes`].
pub fn get_u32(src: &mut Bytes, name: &str) -> Result<u32, std::io::Error> {
	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_u32())
}

/// A utility function to get a [`u64`] from a [`Bytes`].
pub fn get_u64(src: &mut Bytes, name: &str) -> Result<u64, std::io::Error> {
	if src.len() < 8 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_u64())
}

/// A utility function to get a [`i8`] from a [`Bytes`].
pub fn get_i8(src: &mut Bytes, name: &str) -> Result<i8, std::io::Error> {
	if src.len() < 1 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_i8())
}

/// A utility function to get a [`i16`] from a [`Bytes`].
pub fn get_i16(src: &mut Bytes, name: &str) -> Result<i16, std::io::Error> {
	if src.len() < 2 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_i16())
}

/// A utility function to get a [`i32`] from a [`Bytes`].
pub fn get_i32(src: &mut Bytes, name: &str) -> Result<i32, std::io::Error> {
	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_i32())
}

/// A utility function to get a [`i64`] from a [`Bytes`].
pub fn get_i64(src: &mut Bytes, name: &str) -> Result<i64, std::io::Error> {
	if src.len() < 8 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}'", name))) }
	Ok(src.get_i64())
}

/// A utility function to get a [`String`] from a [`Bytes`].
pub fn get_string(src: &mut Bytes, name: &str) -> Result<String, std::io::Error> {
	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' string size", name))) }
	let len = src.get_u32() as usize;
	if src.len() < len { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' string", name))) }
	let s = src.copy_to_bytes(len);
	let s = String::from_utf8(s.to_vec())
		.map_err(|_| io::Error::new(ErrorKind::InvalidInput, format!("'{}' is not a valid UTF-8 string", name)))?;

	Ok(s)
}

/// A utility function to get an [`Option`] from a [`Bytes`].
pub fn get_option<T>(src: &mut Bytes, name: &str, get: impl Fn(&mut Bytes) -> Result<T, std::io::Error>) -> Result<Option<T>, std::io::Error> {
	const NO_VALUE: u8 = 0;
	const HAS_VALUE: u8 = 1;

	if src.len() < 1 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' option tag", name))) }
	let tag = src.get_u8();

	match tag {
		NO_VALUE => Ok(None),
		HAS_VALUE => {
			let val = get(src)?;
			Ok(Some(val))
		},
		_ => Err(io::Error::new(ErrorKind::InvalidInput, "invalid option tag")),
	}
}

/// A utility function to get an [`Vec<Option>`] from a [`Bytes`].
pub fn get_option_array<T>(src: &mut Bytes, name: &str, get: impl Fn(&mut Bytes) -> Result<T, std::io::Error>) -> Result<Vec<Option<T>>, std::io::Error> {
	const NO_VALUE: u8 = 0;
	const HAS_VALUE: u8 = 1;

	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' option array size", name))) }
	let len = src.get_u32() as usize;

	let mut arr = Vec::default();
	for i in 0..len {
		if src.len() < 1 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' at {}. option tag", name, i))) }
		let tag = src.get_u8();
	
		match tag {
			NO_VALUE => arr.push(None),
			HAS_VALUE => {
				let val = get(src)?;
				arr.push(Some(val));
			},
			_ => return Err(io::Error::new(ErrorKind::InvalidInput, format!("invalid option tag at {}", i))),
		}
	}

	Ok(arr)
}

/// A utility function to get a [`Vec`] from a [`Bytes`].
pub fn get_array<T>(src: &mut Bytes, name: &str, get: impl Fn(&mut Bytes) -> Result<T, std::io::Error>) -> Result<Vec<T>, std::io::Error> {
	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' array size", name))) }
	let len = src.get_u32() as usize;

	let mut arr = Vec::default();
	for _ in 0..len {
		let val = get(src)?;
		arr.push(val);
	}

	Ok(arr)
}

/// A utility function to get a [`HashSet`] from a [`Bytes`].
pub fn get_hashset<T: PartialEq + Eq + Hash>(src: &mut Bytes, name: &str, get: impl Fn(&mut Bytes) -> Result<T, std::io::Error>) -> Result<HashSet<T>, std::io::Error> {
	if src.len() < 4 { return Err(io::Error::new(ErrorKind::InvalidInput, format!("expected '{}' hashset size", name))) }
	let len = src.get_u32() as usize;

	let mut hashset = HashSet::default();
	for _ in 0..len {
		let val = get(src)?;
		hashset.insert(val);
	}

	Ok(hashset)
}

/// A utility function to put a [`&str`] into a [`BytesMut`].
pub fn put_str(dst: &mut BytesMut, s: &str) {
	dst.put_u32(s.len() as u32);
	dst.put_slice(s.as_bytes());
}

/// A utility function to put an [`Option`] into a [`BytesMut`].
pub fn put_option<T>(dst: &mut BytesMut, opt: &Option<T>, put: impl Fn(&mut BytesMut, &T)) {
	const NO_VALUE: u8 = 0;
	const HAS_VALUE: u8 = 1;

	match opt {
		Some(val) => {
			dst.put_u8(HAS_VALUE);
			put(dst, &val);
		},
		None => dst.put_u8(NO_VALUE),
	}
}

/// A utility function to put a [`Vec`] into a [`BytesMut`].
pub fn put_array<T>(dst: &mut BytesMut, arr: &[T], put: impl Fn(&mut BytesMut, &T)) {
	dst.put_u32(arr.len() as u32);
	for val in arr {
		put(dst, val);
	}
}

/// A utility function to put a [`HashSet`] into a [`BytesMut`].
pub fn put_hashset<T: PartialEq + Eq + Hash>(dst: &mut BytesMut, hashset: &HashSet<T>, put: impl Fn(&mut BytesMut, &T)) {
	dst.put_u32(hashset.len() as u32);
	for val in hashset {
		put(dst, val);
	}
}

/// A utility function to put a [`Vec<Option>`] into a [`BytesMut`].
pub fn put_option_array<T>(dst: &mut BytesMut, arr: &[Option<T>], put: impl Fn(&mut BytesMut, &T)) {
	const NO_VALUE: u8 = 0;
	const HAS_VALUE: u8 = 1;

	dst.put_u32(arr.len() as u32);
	for opt in arr {
		match opt {
			Some(val) => {
				dst.put_u8(HAS_VALUE);
				put(dst, val);
			},
			None => dst.put_u8(NO_VALUE),
		}
	}
}
