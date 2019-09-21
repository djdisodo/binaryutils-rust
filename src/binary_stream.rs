use std::io::Write;
use std::iter::FromIterator;
use crate::binary::*;

pub struct BinaryStream {
	buffer: Vec<u8>,
	offset: usize,
}

impl BinaryStream {
	pub fn new(buffer : Vec<u8>, offset : usize) -> BinaryStream{
		return BinaryStream {
			buffer,
			offset
		};
	}
	pub fn reset(&mut self) {
		self.rewind();
		self.buffer.flush().unwrap();
	}
	pub fn rewind(&mut self) {
		self.set_offset(0);
	}
	pub fn set_offset(&mut self, offset: usize) {
		self.offset = offset;
	}
	pub fn set_buffer(&mut self, buffer: Vec<u8>, offset: usize) {
		self.buffer = buffer;
		self.set_offset(offset);
	}
	pub fn get_offset(&self) -> usize {
		return self.offset;
	}
	pub fn get_buffer(&self) -> &Vec<u8> {
		return &self.buffer;
	}
	pub fn get(&mut self, len: usize) -> Vec<u8> {
		if self.get_buffer().len() < (self.get_offset() + len) {
			panic!("BinaryDataException: No bytes left to read");
		}
		let mut result: Vec<u8> = Vec::from_iter(self.get_buffer().clone().drain(self.get_offset()..(self.get_offset() + len)));
		result.truncate(len);
		self.offset += len;
		return result;
	}
	pub fn get_remaining(&mut self) -> Vec<u8> {
		if self.get_buffer().len() < self.get_offset() {
			panic!("BinaryDataException: No bytes left to read");
		}

		let v : Vec<u8> = Vec::from_iter(self.get_buffer().clone().drain(self.get_offset()..));
		self.set_offset(self.get_buffer().len());
		return v;
	}
	pub fn put(&mut self, bytes: Vec<u8>) {
		self.buffer.extend(bytes);
	}
	pub fn get_bool(&mut self) -> bool {
		return self.get(1).as_slice()[self.get_offset()] != 0;
	}
	pub fn put_bool(&mut self, v: bool) {
		self.buffer.push(v as u8);
	}
	pub fn get_byte(&mut self) -> u8 {
		return self.get(1).get(0).unwrap().clone();
	}
	pub fn put_byte(&mut self, v: u8) {
		self.buffer.push(v);
	}
	pub fn get_short(&mut self, endian: Endian) -> i16 {
		return read_short(self.get(2), endian);
	}
	pub fn get_unsigned_short(&mut self, endian: Endian) -> u16 {
		return read_unsigned_short(self.get(2), endian);
	}
	pub fn put_short(&mut self, v: i16, endian: Endian) {
		self.put(Vec::from(write_short(v, endian)));
	}
	pub fn put_unsigned_short(&mut self, v: u16, endian: Endian) {
		self.put(Vec::from(write_unsigned_short(v, endian)));
	}
	/* TODO
	pub fn get_triad(&mut self, endian: Endian) -> i32 {
		return read_triad(self.get(3), endian);
	}
	*/
	pub fn get_unsigned_triad(&mut self, endian: Endian) -> u32 {
		return read_unsigned_triad(self.get(3), endian);
	}
	/* TODO
	pub fn put_triad(&mut self, v: i32, endian: Endian) {
		self.put(write_triad(v, endian));
	}
	*/
	pub fn put_unsigned_triad(&mut self, v: u32, endian: Endian) {
		self.put(write_unsigned_triad(v, endian));
	}
	pub fn get_int(&mut self, endian: Endian) -> i32 {
		return read_int(self.get(4), endian);
	}
	pub fn get_unsigned_int(&mut self, endian: Endian) -> u32 {
		return read_unsigned_int(self.get(4), endian);
	}
	pub fn put_int(&mut self, v: i32, endian: Endian) {
		self.put(write_int(v, endian));
	}
	pub fn put_unsigned_int(&mut self, v: u32, endian: Endian) {
		self.put(write_unsigned_int(v, endian));
	}
	pub fn get_float(&mut self, endian: Endian) -> f32 {
		return read_float(self.get(4), endian);
	}
	pub fn put_float(&mut self, v: f32, endian: Endian) {
		self.put(write_float(v, endian));
	}
	pub fn get_double(&mut self, endian: Endian) -> f64 {
		return read_double(self.get(8), endian);
	}
	pub fn get_long(&mut self, endian: Endian) -> i64 {
		return read_long(self.get(8), endian);
	}
	pub fn get_unsigned_long(&mut self, endian: Endian) -> u64 {
		return read_unsigned_long(self.get(8), endian);
	}
	pub fn put_long(&mut self, v: i64, endian: Endian) {
		self.put(write_long(v, endian));
	}
	pub fn put_unsigned_long(&mut self, v: u64, endian: Endian) {
		self.put(write_unsigned_long(v, endian));
	}
	pub fn get_unsigned_var_int(&mut self) -> u32 {
		let result: u32 = read_unsigned_var_int(&mut self.buffer, &mut self.offset);
		return result;
	}
	pub fn put_unsigned_var_int(&mut self, v: u32) {
		self.put(write_unsigned_var_int(v));
	}
	pub fn get_var_int(&mut self) -> i32 {
		let result: i32 = read_var_int(&mut self.buffer, &mut self.offset);
		return result;
	}
	pub fn put_var_int(&mut self, v: i32) {
		self.put(write_var_int(v));
	}
	pub fn get_unsigned_var_long(&mut self) -> u64 {
		let result: u64 = read_unsigned_var_long(&mut self.buffer, &mut self.offset);
		return result;
	}
	pub fn put_unsigned_var_long(&mut self, v: u64) {
		self.put(write_unsigned_var_long(v));
	}
	pub fn get_var_long(&mut self) -> i64 {
		let result: i64 = read_var_long(&mut self.buffer, &mut self.offset);
		return result;
	}
	pub fn put_var_long(&mut self, v: i64) {
		self.put(write_var_long(v));
	}
	pub fn feof(&self) -> bool {
		return self.get_buffer().len() < self.get_offset();
	}
}