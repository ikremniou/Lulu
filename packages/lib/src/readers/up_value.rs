use super::{primitives::read_string, Reader};
use crate::types::{AssemblyHeader, LuaString};
use bytes::Bytes;

pub struct FunctionUpValuesReader;
impl Reader<LuaString> for FunctionUpValuesReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaString, String> {
        return Ok(read_string(buf, header)?.unwrap());
    }
}