use bytes::{Buf, Bytes};

pub struct Disassembly {
    pub header: AssemblyHeader,
}

pub enum LuaVersion {
    Lua51 = 0x51,
}

pub enum LuaFormatVersion {
    Official = 0x00,
}

pub enum LuaEndian {
    BigEndian = 0x00,
    LittleEndian = 0x01,
}

pub enum LuaSizeOfInt {
    Default = 0x04,
}

pub enum LuaSizeOfSizeT {
    Default = 0x04,
}

pub enum LuaSizeOfInstruction {
    Default = 0x04,
}

pub enum LuaSizeOfNumber {
    Default = 0x08,
}

pub enum LuaIntegralFlag {
    Floating = 0x00,
    Integral = 0x01,
}

pub struct AssemblyHeader {
    pub version: LuaVersion,
    pub format_version: LuaFormatVersion,
    pub endian: LuaEndian,
    pub int_size: LuaSizeOfInt,
    pub size_t_size: LuaSizeOfSizeT,
    pub instruction_size: LuaSizeOfInstruction,
    pub number_size: LuaSizeOfNumber,
    pub integral_flag: LuaIntegralFlag
}

pub fn disassemble(raw_buffer: Vec<u8>) -> Disassembly {
    println!("{:x?}", raw_buffer);
    let buffer = Bytes::from(raw_buffer);
    let header = read_header(buffer).unwrap();
    return Disassembly { header };
}

pub fn read_header(mut buffer: Bytes) -> Result<AssemblyHeader, String> {
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
        x => return Err(format!("Header size of instruction is not supported: {:x?}", x)),
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
