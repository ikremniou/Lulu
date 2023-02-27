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

pub enum LuaInteger {
    I32(i32),
}

#[derive(PartialEq, Eq)]
pub enum LuaSizeT {
    U32(u32),
}

pub type LuaString = String;

pub enum LuaIsVarArg {
    HasArg = 0x01,
    IsVarArg = 0x02,
    NeedsArg = 0x03,
}

pub struct LuaFunction {
    pub source_name: Option<LuaString>,
    pub line_defined: LuaInteger,
    pub last_line_defined: LuaInteger,
    pub up_values_count: u8,
    pub params_count: u8,
    pub is_var_arg: LuaIsVarArg,
    pub max_stack_size: u8
}