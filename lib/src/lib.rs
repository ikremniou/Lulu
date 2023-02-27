mod reader;
mod types;

use crate::reader::{read_function, read_header};
use bytes::Bytes;
use types::{AssemblyHeader, LuaFunction};

pub struct Disassembly {
    pub header: AssemblyHeader,
    pub function: LuaFunction,
}

pub fn disassemble(raw_buffer: Vec<u8>) -> Disassembly {
    println!("{:x?}", raw_buffer);
    let mut buffer = Bytes::from(raw_buffer);
    let header = read_header(&mut buffer).unwrap();
    let function: LuaFunction = read_function(&mut buffer, &header).unwrap();
    return Disassembly { header, function };
}
