extern crate byteorder;
use std::io::Cursor;
use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::iter::FromIterator;

pub enum Endian {
	Big = 0,
	Little = 1,
}
pub fn read_short(bytes: Vec<u8>, endian: Endian) -> i16 {
	match endian {
		Endian::Big => return Cursor::read_i16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_i16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_short(bytes: Vec<u8>, endian: Endian) -> u16 {
	match endian {
		Endian::Big => return Cursor::read_u16::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_u16::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_short(v: i16, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_i16::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_i16::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_short(v: u16, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_u16::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_u16::<LittleEndian>(v).unwrap()
	}
	return bytes;
}
/* TODO
pub fn read_triad(bytes: Vec<u8>, endian: Endian) -> i32 {
	let mut bytes: Vec<u8> = Vec::from(bytes);
	match endian {
		Endian::Big => {
			bytes.reverse();
			bytes.push((bytes[2] >> 7) << 7);
			bytes[2] = (bytes[2] << 1) >> 1;
			bytes.reverse();
			return Cursor::read_i32::<BigEndian>(&mut Cursor::new(bytes)).unwrap()
		},
		Endian::Little => {
			bytes.push((bytes[2] >> 7) << 7);
			bytes[2] = (bytes[2] << 1) >> 1;
			return Cursor::read_i32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
		}
	}
}
*/
pub fn read_unsigned_triad(bytes: Vec<u8>, endian: Endian) -> u32 {
	let mut bytes: Vec<u8> = Vec::from(bytes);
	match endian {
		Endian::Big => {
			bytes.reverse();
			bytes.push(0);
			bytes.reverse();
			return Cursor::read_u32::<BigEndian>(&mut Cursor::new(bytes)).unwrap()
		},
		Endian::Little => {
			bytes.push(0);
			return Cursor::read_u32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
		}
	}
}
/* TODO
pub fn write_triad(v: i32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => {
			bytes.write_i32::<BigEndian>(v).unwrap();
			bytes = Vec::from_iter(bytes.drain(..1));
			bytes[0] = (bytes[0] << 1) >> 1;
			bytes[0] = bytes[0] | (((v >> 31) as u8) << 7);
		}
		Endian::Little => {
			bytes.write_i32::<LittleEndian>(v).unwrap();
			bytes.truncate(3);
			bytes[2] = (bytes[2] << 1) >> 1;
			bytes[2] = bytes[2] | (((v >> 31) as u8) << 7);
		}
	}
	return bytes;
}
*/
pub fn write_unsigned_triad(v: u32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => {
			bytes.write_u32::<BigEndian>(v).unwrap();
			bytes = Vec::from_iter(bytes.drain(..1));
		}
		Endian::Little => {
			bytes.write_u32::<LittleEndian>(v).unwrap();
			bytes.truncate(3);
		}
	}
	return bytes;
}

pub fn read_int(bytes: Vec<u8>, endian: Endian) -> i32 {
	match endian {
		Endian::Big => return Cursor::read_i32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_i32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_int(bytes: Vec<u8>, endian: Endian) -> u32 {
	match endian {
		Endian::Big => return Cursor::read_u32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_u32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_int(v: i32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_i32::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_i32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_int(v: u32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_u32::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_u32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_float(bytes: Vec<u8>, endian: Endian) -> f32 {
	match endian {
		Endian::Big => return Cursor::read_f32::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_f32::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_float(v: f32, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_f32::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_f32::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_double(bytes: Vec<u8>, endian: Endian) -> f64 {
	match endian {
		Endian::Big => return Cursor::read_f64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_f64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_double(v: f64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_f64::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_f64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_long(bytes: Vec<u8>, endian: Endian) -> i64 {
	match endian {
		Endian::Big => return Cursor::read_i64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_i64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn read_unsigned_long(bytes: Vec<u8>, endian: Endian) -> u64 {
	match endian {
		Endian::Big => return Cursor::read_u64::<BigEndian>(&mut Cursor::new(bytes)).unwrap(),
		Endian::Little => return Cursor::read_u64::<LittleEndian>(&mut Cursor::new(bytes)).unwrap()
	}
}

pub fn write_long(v: i64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_i64::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_i64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn write_unsigned_long(v: u64, endian: Endian) -> Vec<u8> {
	let mut bytes: Vec<u8> = Vec::new();
	match endian {
		Endian::Big => bytes.write_u64::<BigEndian>(v).unwrap(),
		Endian::Little => bytes.write_u64::<LittleEndian>(v).unwrap()
	}
	return bytes;
}

pub fn read_var_int(buffer: &Vec<u8>, offset : &mut usize) -> i32 {
	let raw: i32 = read_unsigned_var_int(&buffer, offset) as i32;
	let temp: i32 = (((raw << 31) >> 31) ^ raw) >> 1;
	return temp ^ (raw & (1 << 31));
}

pub fn read_unsigned_var_int(buffer: &Vec<u8>, offset : &mut usize) -> u32 {
	let mut v: u32 = 0;
	if buffer.len() <= *offset as usize{
		panic!("BinaryDataException: No bytes left to read");
	}
	let mut i : u8 = 0;
	while i < 28 {
		if buffer.len() <= *offset as usize{
			panic!("BinaryDataException: No bytes left to read");
		}
		let b : u8 = buffer[*offset];
		*offset += 1;
		v |= ((b & 0x7f) as u32) << (i as u32);
		if (b & 0x80) == 0 {
			return v;
		}
		i += 7;
	}
	panic!("BinaryDataException : VarInt did not terminate after 5 bytes!");
}

pub fn write_var_int(v: i32) -> Vec<u8> {
	return write_unsigned_var_int((v << 1 ^ (v >> 31)) as u32);
}

pub fn write_unsigned_var_int(mut v: u32) -> Vec<u8> {
	let mut buf: Vec<u8> = Vec::new();
	v &= 0xffffffff;
	for _i in 0..5 {
		if (v >> 7) != 0 {
			buf.push((v | 0x80) as u8);
		} else {
			buf.push((v & 0x7f) as u8);
			return buf;
		}
		v >>= 7;
	}
	panic!("InvalidArgumentException : Value too large to ve encoded as a VarInt");
}

pub fn read_var_long(buffer: &Vec<u8>, offset : &mut usize) -> i64 {
	let raw: i64 = read_unsigned_var_long(&buffer, offset) as i64;
	let temp: i64 = (((raw << 63) >> 63) ^ raw) >> 1;
	return temp ^ (raw & (1 << 63));
}

pub fn read_unsigned_var_long(buffer: &Vec<u8>, offset : &mut usize) -> u64 {
	let mut v: u64 = 0;
	let mut i : u8 = 0;
	while i <= 63 {
		if buffer.len() <= *offset as usize{
			panic!("BinaryDataException: No bytes left to read");
		}
		let b : u8 = buffer[*offset];
		*offset += 1;
		v |= ((b & 0x7f) as u64) << (i as u64);
		if (b & 0x80) == 0 {
			return v;
		}
		i += 7;
	}
	panic!("BinaryDataException : VarInt did not terminate after 10 bytes!");
}

pub fn write_var_long(v: i64) -> Vec<u8> {
	return write_unsigned_var_long((v << 1 ^ (v >> 63)) as u64);
}

pub fn write_unsigned_var_long(mut v: u64) -> Vec<u8> {
	let mut buf: Vec<u8> = Vec::new();
	for _i in 0..10 {
		if (v >> 7) != 0 {
			buf.push((v | 0x80) as u8);
		} else {
			buf.push((v & 0x7f) as u8);
			return buf;
		}
		v >>= 7;
	}
	panic!("InvalidArgumentException : Value too large to ve encoded as a VarInt")
}