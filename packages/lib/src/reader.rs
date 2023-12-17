use bytes::{Buf, Bytes};

use crate::types::{
    AssemblyHeader, LuaConstant, LuaEndian, LuaFormatVersion, LuaFunction, LuaFunctionArgType,
    LuaInstruction, LuaInteger, LuaIntegralFlag, LuaLocal, LuaNumber, LuaOpCode, LuaSizeT,
    LuaString, LuaVersion,
};

trait ListReader<T> {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<T, String>;
    fn read_list(buf: &mut Bytes, header: &AssemblyHeader) -> Result<Vec<T>, String> {
        let list_size = read_integer(buf, header)?;
        let real_size: usize = match list_size {
            LuaInteger::I32(x) => x.try_into().unwrap(),
        };

        if real_size == 0 {
            return Ok(vec![]);
        }

        let mut result = Vec::with_capacity(real_size);
        for _ in 0..real_size {
            let item = Self::read_list_item(buf, header)?;
            result.push(item);
        }

        return Ok(result);
    }
}

struct InstructionsReader;
impl ListReader<LuaInstruction> for InstructionsReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInstruction, String> {
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

struct ConstantReader;
impl ListReader<LuaConstant> for ConstantReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaConstant, String> {
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

struct FunctionPrototypeReader;
impl ListReader<LuaFunction> for FunctionPrototypeReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaFunction, String> {
        return read_function(buf, header);
    }
}

struct FunctionLinePositionsReader;
impl ListReader<LuaInteger> for FunctionLinePositionsReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaInteger, String> {
        return read_integer(buf, header);
    }
}

struct FunctionLocalListReader;
impl ListReader<LuaLocal> for FunctionLocalListReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaLocal, String> {
        let name = read_string(buf, header)?.unwrap();
        let start = read_integer(buf, header)?;
        let end = read_integer(buf, header)?;
        return Ok(LuaLocal { name, start, end });
    }
}

struct FunctionUpValuesReader;
impl ListReader<LuaString> for FunctionUpValuesReader {
    fn read_list_item(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaString, String> {
        return Ok(read_string(buf, header)?.unwrap());
    }
}

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

pub fn read_function(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaFunction, String> {
    let source_name = read_string(buf, header)?;
    let line_defined = read_integer(buf, header)?;
    let last_line_defined = read_integer(buf, header)?;

    let up_values_count = buf.get_u8();
    let params_count = buf.get_u8();
    let is_var_arg = match buf.get_u8() {
        0x00 => LuaFunctionArgType::NormalFunc,
        0x02 => LuaFunctionArgType::MainOrCompat,
        0x03 => LuaFunctionArgType::SpreadArg,
        0x07 => LuaFunctionArgType::OldVarArg,
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
        instructions: InstructionsReader::read_list(buf, header)?,
        constants: ConstantReader::read_list(buf, header)?,
        prototypes: FunctionPrototypeReader::read_list(buf, header)?,
        line_positions: FunctionLinePositionsReader::read_list(buf, header)?,
        locals: FunctionLocalListReader::read_list(buf, header)?,
        up_values: FunctionUpValuesReader::read_list(buf, header)?,
    });
}
