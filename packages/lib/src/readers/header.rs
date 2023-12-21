use crate::types::{AssemblyHeader, LuaEndian, LuaFormatVersion, LuaIntegralFlag, LuaVersion};
use bytes::{Buf, Bytes};

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
        0x04 => 0x04,
        x => return Err(format!("Header size of int is not supported: {:x?}", x)),
    };

    let size_t_size = match buffer.get_u8() {
        0x04 => 0x04,
        0x08 => 0x08,
        x => return Err(format!("Header size of size_t is not supported: {:x?}", x)),
    };

    let instruction_size = match buffer.get_u8() {
        0x04 => 0x04,
        x => {
            return Err(format!(
                "Header size of instruction is not supported: {:x?}",
                x
            ))
        }
    };

    let number_size = match buffer.get_u8() {
        0x08 => 0x08,
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
