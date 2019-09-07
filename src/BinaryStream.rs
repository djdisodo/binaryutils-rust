use std::io::{Write, Cursor};
use std::ops::{Add, AddAssign};
use std::fmt::Error;
use std::convert::TryInto;
use std::iter::FromIterator;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt, LittleEndian};
use crate::binary::*;

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
	fn get_short(&mut self, endian : Endian) -> i16 {
		return read_short(self.get(2), endian);
	}
	fn get_unsigned_short(&mut self, endian : Endian) -> u16 {
		return read_unsigned_short(self.get(2), endian);
	}
	fn put_short(&mut self, v : i16, endian : Endian) {
		self.put(Vec::from(write_short(v, endian)));
	}
	fn put_unsigned_short(&mut self, v : u16, endian : Endian) {
		self.put(Vec::from(write_unsigned_short(v, endian)));
	}
	fn get_triad(&mut self, endian : Endian) -> i32 {
		return read_triad(self.get(3), endian);
	}
	fn get_unsigned_triad(&mut self, endian : Endian) -> u32 {
		return read_unsigned_triad(self.get(3), endian);
	}
	fn put_triad(&mut self, v : i32, endian : Endian) {
		self.put(write_triad(v, endian).to_vec());
	}
	fn put_unsigned_triad(&mut self, v : u32, endian : Endian) {
		self.put(write_unsigned_triad(v, endian).to_vec());
	}
	fn get_int(&mut self, endian : Endian) -> i32 {
		return read_int(self.get(4), endian);
	}
	fn get_unsigned_int(&mut self, endian : Endian) -> u32 {
		return read_unsigned_int(self.get(4), endian);
	}
	fn put_int(&mut self, v : i32, endian : Endian) {
		self.put(write_int(v, endian).to_vec());
	}
	fn put_unsigned_int(&mut self, v : u32, endian : Endian) {
		self.put(write_unsigned_int(v, endian).to_vec());
	}
	fn get_float(&mut self, endian : Endian) -> f32 {
		return read_float(self.get(4), endian);
	}
	fn put_float(&mut self, v : f32, endian : Endian) {
		self.put(write_float(v, endian).to_vec());
	}
	fn get_double(&mut self, endian : Endian) -> f64 {
		return read_double(self.get(8), endian);
	}
	fn get_long(&mut self, endian : Endian) -> i64 {
		return read_long(self.get(8), endian);
	}
	fn get_unsigned_long(&mut self, endian : Endian) -> u64 {
		return read_unsigned_long(self.get(8), endian);
	}
	fn put_long(&mut self, v : i64, endian : Endian) {
		self.put(write_long(v, endian).to_vec());
	}
	fn put_unsigned_long(&mut self, v : u64, endian : Endian) {
		self.put(write_unsigned_long(v, endian).to_vec());
	}
	fn get_unsigned_var_int(&mut self, endian : Endian) -> u32 {
		let mut read_bytes  : u8 = 0;
		let result : u32 = read_unsigned_var_int(Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset())), &mut read_bytes);
		self.offset.add(read_bytes as usize);
		return result;
	}
	fn put_unsigned_var_int(&mut self, v : u32, endian : Endian) {
		self.put(write_unsigned_var_int(v).to_vec());
	}
	fn get_var_int(&mut self, endian : Endian) -> i32 {
		let mut read_bytes  : u8 = 0;
		let result : i32 = read_var_int(Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset())), &mut read_bytes);
		self.offset.add(read_bytes as usize);
		return result;
	}
	fn put_var_int(&mut self, v : i32, endian : Endian) {
		self.put(write_var_int(v).to_vec());
	}
	fn get_unsigned_var_long(&mut self, endian : Endian) -> u64 {
		let mut read_bytes  : u8 = 0;
		let result : u64 = read_unsigned_var_long(Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset())), &mut read_bytes);
		self.offset.add(read_bytes as usize);
		return result;
	}
	fn put_unsigned_var_long(&mut self, v : u64, endian : Endian) {
		self.put(write_unsigned_var_long(v).to_vec());
	}
	fn get_var_long(&mut self, endian : Endian) -> i64 {
		let mut read_bytes  : u8 = 0;
		let result : i64 = read_var_long(Vec::from_iter(self.get_buffer().clone().drain(..self.get_offset())), &mut read_bytes);
		self.offset.add(read_bytes as usize);
		return result;
	}
	fn put_var_long(&mut self, v : i64, endian : Endian) {
		self.put(write_var_long(v).to_vec());
	}
	fn feof(&self) -> bool{
		return self.get_buffer().len() < self.get_offset();
	}
}