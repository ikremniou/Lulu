use bytes::{Buf, Bytes};

use crate::types::{
    AssemblyHeader, LuaEndian, LuaFormatVersion, LuaFunction, LuaInteger, LuaIntegralFlag,
    LuaIsVarArg, LuaSizeOfInstruction, LuaSizeOfInt, LuaSizeOfNumber, LuaSizeOfSizeT, LuaSizeT,
    LuaString, LuaVersion,
};

pub fn read_header(buffer: &mut Bytes) -> Result<AssemblyHeader, String> {
    let signature = match buffer.get(0..4) {
        Some(s) => s,
        None => {
            return Err("Cannot read assembly header. Header signature out of range.".to_owned())
        }
    };

    if signature != b"\x1BLua" {
        return Err(format!("Header signature is not valid: {:x?}", signature));
    }

    buffer.advance(4);
    let version = match buffer.get_u8() {
        0x51 => LuaVersion::Lua51,
        x => return Err(format!("Header version is not valid: {:x?}", x)),
    };

    let format_version = match buffer.get_u8() {
        0x00 => LuaFormatVersion::Official,
        x => return Err(format!("Header format version is not supported: {:x?}", x)),
    };

    let endian = match buffer.get_u8() {
        0x00 => LuaEndian::BigEndian,
        0x01 => LuaEndian::LittleEndian,
        x => return Err(format!("Header endian is not valid: {:x?}", x)),
    };

    let int_size = match buffer.get_u8() {
        0x04 => LuaSizeOfInt::Default,
        x => return Err(format!("Header size of int is not supported: {:x?}", x)),
    };

    let size_t_size = match buffer.get_u8() {
        0x04 => LuaSizeOfSizeT::Default,
        x => return Err(format!("Header size of size_t is not supported: {:x?}", x)),
    };

    let instruction_size = match buffer.get_u8() {
        0x04 => LuaSizeOfInstruction::Default,
        x => {
            return Err(format!(
                "Header size of instruction is not supported: {:x?}",
                x
            ))
        }
    };

    let number_size = match buffer.get_u8() {
        0x08 => LuaSizeOfNumber::Default,
        x => return Err(format!("Header size of number is not supported: {:x?}", x)),
    };

    let integral_flag = match buffer.get_u8() {
        0x00 => LuaIntegralFlag::Floating,
        0x01 => LuaIntegralFlag::Integral,
        x => return Err(format!("Header integral flag is not valid: {:x?}", x)),
    };

    return Ok(AssemblyHeader {
        version,
        format_version,
        endian,
        int_size,
        size_t_size,
        instruction_size,
        number_size,
        integral_flag,
    });
}

pub fn read_integer(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInteger, String> {
    match (&header.int_size, &header.endian) {
        (LuaSizeOfInt::Default, LuaEndian::LittleEndian) => Ok(LuaInteger::I32(buf.get_i32_le())),
        (LuaSizeOfInt::Default, LuaEndian::BigEndian) => Ok(LuaInteger::I32(buf.get_i32())),
    }
}

pub fn read_size_t(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaSizeT, String> {
    match (&header.size_t_size, &header.endian) {
        (LuaSizeOfSizeT::Default, LuaEndian::LittleEndian) => Ok(LuaSizeT::U32(buf.get_u32_le())),
        (LuaSizeOfSizeT::Default, LuaEndian::BigEndian) => Ok(LuaSizeT::U32(buf.get_u32())),
    }
}

pub fn read_string(buf: &mut Bytes, header: &AssemblyHeader) -> Result<Option<LuaString>, String> {
    let str_size = read_size_t(buf, header)?;
    let size: usize = match str_size {
        LuaSizeT::U32(x) => x as usize,
    };

    if size == 0 {
        return Ok(None);
    }

    let string_bytes = buf.get(0..size - 1).unwrap();
    let string = String::from_utf8_lossy(string_bytes).to_string();

    buf.advance(size);
    return Ok(Some(string));
}

pub fn read_function(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaFunction, String> {
    let source_name = read_string(buf, header)?;
    let line_defined = read_integer(buf, header)?;
    let last_line_defined = read_integer(buf, header)?;

    let up_values_count = buf.get_u8();
    let params_count = buf.get_u8();
    let is_var_arg = match buf.get_u8() {
        0x01 => LuaIsVarArg::HasArg,
        0x02 => LuaIsVarArg::IsVarArg,
        0x03 => LuaIsVarArg::NeedsArg,
        x => return Err(format!("Invalid value found for 'var_arg': {:x?}", x)),
    };
    let max_stack_size = buf.get_u8();

    return Ok(LuaFunction {
        source_name,
        line_defined,
        last_line_defined,
        up_values_count,
        params_count,
        is_var_arg,
        max_stack_size,
    });
}
