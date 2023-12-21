use crate::types::{AssemblyHeader, LuaEndian, LuaInteger, LuaNumber, LuaSizeT, LuaString};
use bytes::{Bytes, Buf};

pub fn read_integer(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInteger, String> {
    match (header.int_size, &header.endian) {
        (0x04, LuaEndian::LittleEndian) => Ok(LuaInteger::I32(buf.get_i32_le())),
        (0x04, LuaEndian::BigEndian) => Ok(LuaInteger::I32(buf.get_i32())),
        _ => Err("Cannot read integer".to_owned()),
    }
}

pub fn read_number(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaNumber, String> {
    match (header.int_size, &header.endian) {
        (0x04, LuaEndian::LittleEndian) => Ok(buf.get_f64_le()),
        (0x04, LuaEndian::BigEndian) => Ok(buf.get_f64()),
        _ => Err("Cannot read integer".to_owned()),
    }
}

pub fn read_size_t(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaSizeT, String> {
    match (header.size_t_size, &header.endian) {
        (0x04, LuaEndian::LittleEndian) => Ok(LuaSizeT::U32(buf.get_u32_le())),
        (0x04, LuaEndian::BigEndian) => Ok(LuaSizeT::U32(buf.get_u32())),
        (0x08, LuaEndian::LittleEndian) => Ok(LuaSizeT::U64(buf.get_u64_le())),
        (0x08, LuaEndian::BigEndian) => Ok(LuaSizeT::U64(buf.get_u64())),
        _ => Err("Cannot read size_t".to_owned()),
    }
}

pub fn read_string(buf: &mut Bytes, header: &AssemblyHeader) -> Result<Option<LuaString>, String> {
    let str_size = read_size_t(buf, header)?;
    let size: usize = match str_size {
        LuaSizeT::U32(x) => x as usize,
        LuaSizeT::U64(x) => x as usize,
    };

    if size == 0 {
        return Ok(None);
    }

    let string_bytes = buf.get(0..size - 1).unwrap();
    let string = String::from_utf8_lossy(string_bytes).to_string();

    buf.advance(size);
    return Ok(Some(string));
}
