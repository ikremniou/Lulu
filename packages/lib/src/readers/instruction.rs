use crate::types::{LuaOpCode, LuaInstruction, LuaEndian, AssemblyHeader};
use super::Reader;
use bytes::{Bytes, Buf};

pub struct InstructionsReader;
impl Reader<LuaInstruction> for InstructionsReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInstruction, String> {
        let instruction_raw = match header.endian {
            LuaEndian::BigEndian => buf.get_u32(),
            LuaEndian::LittleEndian => buf.get_u32_le(),
        };

        let op_code_raw = instruction_raw << 26 >> 26;
        let registers = instruction_raw >> 6;
        let op_code = match op_code_raw {
            0x00 => Ok(LuaOpCode::MOVE),
            0x01 => Ok(LuaOpCode::LOADK),
            0x02 => Ok(LuaOpCode::LOADBOOL),
            0x03 => Ok(LuaOpCode::LOADNIL),
            0x04 => Ok(LuaOpCode::GETUPVAL),
            0x05 => Ok(LuaOpCode::GETGLOBAL),
            0x06 => Ok(LuaOpCode::GETTABLE),
            0x07 => Ok(LuaOpCode::SETGLOBAL),
            0x08 => Ok(LuaOpCode::SETUPVAL),
            0x09 => Ok(LuaOpCode::SETTABLE),
            0x0a => Ok(LuaOpCode::NEWTABLE),
            0x0b => Ok(LuaOpCode::SELF),
            0x0c => Ok(LuaOpCode::ADD),
            0x0d => Ok(LuaOpCode::SUB),
            0x0e => Ok(LuaOpCode::MUL),
            0x0f => Ok(LuaOpCode::DIV),
            0x10 => Ok(LuaOpCode::MOD),
            0x11 => Ok(LuaOpCode::POW),
            0x12 => Ok(LuaOpCode::UNM),
            0x13 => Ok(LuaOpCode::NOT),
            0x14 => Ok(LuaOpCode::LEN),
            0x15 => Ok(LuaOpCode::CONCAT),
            0x16 => Ok(LuaOpCode::JMP),
            0x17 => Ok(LuaOpCode::EQ),
            0x18 => Ok(LuaOpCode::LT),
            0x19 => Ok(LuaOpCode::LE),
            0x1a => Ok(LuaOpCode::TEST),
            0x1b => Ok(LuaOpCode::TESTSET),
            0x1c => Ok(LuaOpCode::CALL),
            0x1d => Ok(LuaOpCode::TAILCALL),
            0x1e => Ok(LuaOpCode::RETURN),
            0x1f => Ok(LuaOpCode::FORLOOP),
            0x20 => Ok(LuaOpCode::FORPREP),
            0x21 => Ok(LuaOpCode::TFORLOOP),
            0x22 => Ok(LuaOpCode::SETLIST),
            0x23 => Ok(LuaOpCode::CLOSE),
            0x24 => Ok(LuaOpCode::CLOSURE),
            0x25 => Ok(LuaOpCode::VARARG),
            _ => Err(format!("Invalid OpCode detected: {}", op_code_raw)),
        }?;

        return Ok(LuaInstruction { op_code, registers });
    }
}