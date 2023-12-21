use super::{Reader, primitives::read_number, primitives::read_string};
use crate::types::{AssemblyHeader, LuaConstant};
use bytes::{Buf, Bytes};

pub struct ConstantReader;
impl Reader<LuaConstant> for ConstantReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaConstant, String> {
        let cont_type: u8 = buf.get_u8();
        return match cont_type {
            0x00 => Ok(LuaConstant::Nil()),
            0x01 => Ok(LuaConstant::Bool(buf.get_u8() != 0)),
            0x03 => Ok(LuaConstant::Number(read_number(buf, header)?)),
            0x04 => Ok(LuaConstant::String(read_string(buf, header)?.unwrap())),
            x => return Err(format!("LuaConstant type {:x?} is unknown", x)),
        };
    }
}
