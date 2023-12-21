use super::{primitives::read_integer, primitives::read_string, Reader};
use crate::types::{AssemblyHeader, LuaLocal};
use bytes::Bytes;

pub struct FunctionLocalListReader;
impl Reader<LuaLocal> for FunctionLocalListReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaLocal, String> {
        let name = read_string(buf, header)?.unwrap();
        let start = read_integer(buf, header)?;
        let end = read_integer(buf, header)?;
        return Ok(LuaLocal { name, start, end });
    }
}
