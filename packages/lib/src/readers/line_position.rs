use super::{primitives::read_integer, Reader};
use crate::types::{AssemblyHeader, LuaInteger};
use bytes::Bytes;

pub struct FunctionLinePositionsReader;
impl Reader<LuaInteger> for FunctionLinePositionsReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInteger, String> {
        return read_integer(buf, header);
    }
}
