extern crate byteorder;
use byteorder::BigEndian;
use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::any::{Any, TypeId};
use crate::binary::Endian::{Big, Little};

pub enum Endian{
	Big = 0,
	Little = 1
}
pub fn read_short(bytes : Vec<u8>, endian : Endian) -> u16{
	match endian {
		Big => return Cursor::read_u16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_u16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}
pub fn read_signed_short(bytes : Vec<u8>, endian : Endian) -> i16{
	match endian {
		Big => return Cursor::read_i16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}
pub fn write_short(v : u16, endian : Endian) -> Vec<u8>{
	let mut bytes : Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_u16::<BigEndian>(v).unwrap(),
		Little => bytes.write_u16::<LittleEndian>(v).unwrap()
	}
	return bytes;
}
pub fn read_triad(bytes : Vec<u8>, endian : Endian) -> u32{
	let mut bytes : Vec<u8> = Vec::from(bytes);
	match endian {
		Little => bytes.reverse(),
		_ => {}
	}
	bytes.push(0);
	match endian {
		Big => return Cursor::read_u32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_u32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}
pub fn write_triad(v : u32, endian : Endian) -> Vec<u8>{
	let mut bytes : Vec<u8> = Vec::new();
	match endian {
		Big => {
			bytes.write_u32::<BigEndian>(v).unwrap();
			bytes.resize(3, 0);
		},
		Little => {
			bytes.write_u32::<LittleEndian>(v).unwrap();
			bytes.reverse();
			bytes.resize(3, 0);
			bytes.reverse();
		}
	}
	return bytes;
}
pub fn read_int(bytes : Vec<u8>, endian : Endian) -> i32{
	match endian {
		Big => return Cursor::read_i32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}
pub fn write_int(v : i32, endian : Endian) -> Vec<u8>{
	let mut bytes : Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_i32::<BigEndian>(v).unwrap(),
		Little => bytes.write_i32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}