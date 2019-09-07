extern crate byteorder;

use std::io::Cursor;
use std::ops::{BitOrAssign, Add};
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

#[derive(Clone, Copy)]
pub enum Endian {
	Big = 0,
	Little = 1,
}

pub fn read_short(bytes: Vec<u8>, endian: Endian) -> i16 {
	match endian {
		Big => return Cursor::read_i16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_short(bytes: Vec<u8>, endian: Endian) -> u16 {
	match endian {
		Big => return Cursor::read_u16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_u16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_short(v: i16, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_i16::<BigEndian>(v).unwrap(),
		Little => bytes.write_i16::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_short(v: u16, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_u16::<BigEndian>(v).unwrap(),
		Little => bytes.write_u16::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_triad(bytes: Vec<u8>, endian: Endian) -> i32 {
	let mut bytes: Vec<u8> = Vec::from(bytes);
	match endian {
		Little => bytes.reverse(),
		_ => {}
	}
	bytes.push(0);
	match endian {
		Big => return Cursor::read_i32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_triad(bytes: Vec<u8>, endian: Endian) -> u32 {
	let mut bytes: Vec<u8> = Vec::from(bytes);
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

pub fn write_triad(v: i32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => {
			bytes.write_i32::<BigEndian>(v).unwrap();
			bytes.resize(3, 0);
		}
		Little => {
			bytes.write_i32::<LittleEndian>(v).unwrap();
			bytes.reverse();
			bytes.resize(3, 0);
			bytes.reverse();
		}
	}
	return bytes;
}

pub fn write_unsigned_triad(v: u32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => {
			bytes.write_u32::<BigEndian>(v).unwrap();
			bytes.resize(3, 0);
		}
		Little => {
			bytes.write_u32::<LittleEndian>(v).unwrap();
			bytes.reverse();
			bytes.resize(3, 0);
			bytes.reverse();
		}
	}
	return bytes;
}

pub fn read_int(bytes: Vec<u8>, endian: Endian) -> i32 {
	match endian {
		Big => return Cursor::read_i32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_int(bytes: Vec<u8>, endian: Endian) -> u32 {
	match endian {
		Big => return Cursor::read_u32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_u32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_int(v: i32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_i32::<BigEndian>(v).unwrap(),
		Little => bytes.write_i32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_int(v: u32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_u32::<BigEndian>(v).unwrap(),
		Little => bytes.write_u32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_float(bytes: Vec<u8>, endian: Endian) -> f32 {
	match endian {
		Big => return Cursor::read_f32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_f32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_float(v: f32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_f32::<BigEndian>(v).unwrap(),
		Little => bytes.write_f32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_double(bytes: Vec<u8>, endian: Endian) -> f64 {
	match endian {
		Big => return Cursor::read_f64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_f64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_double(v: f64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_f64::<BigEndian>(v).unwrap(),
		Little => bytes.write_f64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_long(bytes: Vec<u8>, endian: Endian) -> i64 {
	match endian {
		Big => return Cursor::read_i64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_i64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_long(bytes: Vec<u8>, endian: Endian) -> u64 {
	match endian {
		Big => return Cursor::read_u64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Little => return Cursor::read_u64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_long(v: i64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_i64::<BigEndian>(v).unwrap(),
		Little => bytes.write_i64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_long(v: u64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Big => bytes.write_u64::<BigEndian>(v).unwrap(),
		Little => bytes.write_u64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_var_int(bytes: Vec<u8>, read_bytes : &mut u8 /* stores counts of bytes readed (starts from zero) */) -> i32 {
	let raw: u32 = read_unsigned_var_int(bytes, read_bytes);
	let temp: u32 = (((raw << 31) >> 31) ^ raw) >> 1;
	return (temp ^ (raw & (1 << 31))) as i32;
}

pub fn read_unsigned_var_int(bytes: Vec<u8>, read_bytes : &mut u8 /* stores counts of bytes readed (starts from zero) */) -> u32 {
	let mut v: u32 = 0;
	for i in 0..5 {
		if bytes.len() < i {
			panic!("BinaryDataException: No bytes left to read");
		}
		v.bitor_assign(((bytes.get(i).unwrap().clone() as u32) & 0x7f) << (i * 7) as u32);
		if bytes.get(i).unwrap() & 0x80 == 0 {
			read_bytes.add(i as u8);
			return v;
		}
	}
	panic!("BinaryDataException : VarInt did not terminate after 5 bytes!");
}

pub fn write_var_int(v: i32) -> Vec<u8> {
	return write_unsigned_var_int((v << 1 ^ (v >> 31)) as u32);
}

pub fn write_unsigned_var_int(mut v: u32) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	for i in 0..5 {
		if (v >> 7) != 0 {
			bytes.push((v | 0x80) as u8);
		} else {
			bytes.push((v | 0x7f) as u8);
			return bytes;
		}
		v >>= 7;
	}
	panic!("InvalidArgumentException : Value too large to ve encoded as a VarInt")
}

pub fn read_var_long(bytes: Vec<u8>, read_bytes : &mut u8 /* stores counts of bytes readed (starts from zero) */) -> i64 {
	let raw: u64 = read_unsigned_var_long(bytes, read_bytes);
	let temp: u64 = (((raw << 63) >> 63) ^ raw) >> 1;
	return (temp ^ (raw & (1 << 63))) as i64;
}

pub fn read_unsigned_var_long(bytes: Vec<u8>, read_bytes : &mut u8 /* stores counts of bytes readed (starts from zero) */) -> u64 {
	let mut v: u64 = 0;
	for i in 0..10 {
		if bytes.len() < i {
			panic!("BinaryDataException: No bytes left to read");
		}
		v.bitor_assign(((bytes.get(i).unwrap().clone() as u64) & 0x7f) << (i * 7) as u64);
		if bytes.get(i).unwrap() & 0x80 == 0 {
			read_bytes.add(i as u8);
			return v;
		}
	}
	panic!("BinaryDataException : VarInt did not terminate after 10 bytes!");
}

pub fn write_var_long(v: i64) -> Vec<u8> {
	return write_unsigned_var_long((v << 1 ^ (v >> 63)) as u64);
}

pub fn write_unsigned_var_long(mut v: u64) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	for i in 0..10 {
		if (v >> 7) != 0 {
			bytes.push((v | 0x80) as u8);
		} else {
			bytes.push((v | 0x7f) as u8);
			return bytes;
		}
		v >>= 7;
	}
	panic!("InvalidArgumentException : Value too large to ve encoded as a VarInt")
}