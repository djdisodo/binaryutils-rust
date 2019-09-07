use std::io::{Write, Cursor};
use std::ops::{Add, AddAssign};
use std::fmt::Error;
use std::convert::TryInto;
use std::iter::FromIterator;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt, LittleEndian};
use crate::binary::{read_short, read_signed_short, write_short, write_triad, read_triad, read_int, Endian};

struct BinaryStream {
	offset : usize,
	buffer : Vec<u8>
}
impl BinaryStream {
	fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return BinaryStream {
			buffer,
			offset
		};
	}
	fn reset(&mut self) {
		self.rewind();
		self.buffer.flush().unwrap();
	}
	fn rewind(&mut self) {
		self.set_offset(0);
	}
	fn set_offset(&mut self, offset : usize) {
		self.offset = offset;
	}
	fn set_buffer(&mut self, buffer : Vec<u8>, offset : usize) {
		self.buffer = buffer;
		self.set_offset(offset);
	}
	fn get_offset(&self) -> usize {
		return self.offset;
	}
	fn get_buffer(&self) -> &Vec<u8> {
		return &self.buffer;
	}
	fn get(&mut self, len : usize) -> Vec<u8> {
		if self.get_buffer().len() < (self.get_offset() + len) {
	panic!("BinaryDataException: No bytes left to read");
		}
		let mut result : Vec<u8> = Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset()));
		result.resize(len, 0);
		self.offset.add_assign(1);
		return result;
	}
	fn get_remaining(&mut self) -> Vec<u8> {
		if self.get_buffer().len() < self.get_offset() {
			panic!("BinaryDataException: No bytes left to read");
		}
		self.set_offset(self.get_buffer().len());
		return Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset()));
	}
	fn put(&mut self, bytes : Vec<u8>) {
		self.buffer.extend(bytes);
	}
	fn get_bool(&mut self) -> bool {
		return self.get(1).as_slice()[self.get_offset()] != 0;
	}
	fn put_bool(&mut self, v : bool) {
		self.buffer.push(v as u8);
	}
	fn get_byte(&mut self) -> u8 {
		return self.get(1).get(0).unwrap().clone();
	}
	fn put_byte(&mut self, v : u8) {
		self.buffer.push(v);
	}
	fn get_short<T>(&mut self, endian : Endian) -> u16 {
		return read_short(self.get(2), endian);
	}
	fn get_signed_short<T>(&mut self, endian : Endian) -> i16 {
		return read_signed_short(self.get(2), endian)
	}
	fn put_short<T>(&mut self, v : u16, endian : Endian) {
		self.put(Vec::from(write_short(v, endian)));
	}
	fn get_triad<T>(&mut self, endian : Endian) -> u32 {
		return read_triad(self.get(3), endian);
	}
	fn put_triad<T>(&mut self, v : u32, endian : Endian) {
		self.put(write_triad(v, endian).to_vec());
	}
	fn get_int<T>(&mut self, endian : Endian) -> i32 {
		return read_int(self.get(4), endian);
	}
}