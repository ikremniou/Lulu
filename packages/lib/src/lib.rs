mod readers;
mod error;
pub mod types;

use bytes::Bytes;
use readers::{header::read_header, function::FunctionReader, Reader};
use types::{AssemblyHeader, LuaFunction};

pub struct Disassembly {
    pub header: AssemblyHeader,
    pub function: LuaFunction,
}

pub fn disassemble(raw_buffer: Vec<u8>) -> Result<Disassembly, error::LuluError> {
    let mut buffer = Bytes::from(raw_buffer);
    let header = read_header(&mut buffer).unwrap();
    let function: LuaFunction = FunctionReader::read(&mut buffer, &header)?;
    return Ok(Disassembly { header, function });
}
