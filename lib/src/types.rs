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

pub enum LuaIntegralFlag {
    Floating = 0x00,
    Integral = 0x01,
}

pub struct AssemblyHeader {
    pub version: LuaVersion,
    pub format_version: LuaFormatVersion,
    pub endian: LuaEndian,
    pub int_size: usize,
    pub size_t_size: usize,
    pub instruction_size: usize,
    pub number_size: usize,
    pub integral_flag: LuaIntegralFlag,
}

pub enum LuaInteger {
    I32(i32),
}

pub enum LuaSizeT {
    U32(u32),
}

pub type LuaString = String;
pub type LuaNumber = f64;

pub enum LuaFunctionArgType {
    NormalFunc = 0x00,
    MainOrCompat = 0x02,
    SpreadArg = 0x03,
    OldVarArg = 0x07,
}

pub enum LuaConstant {
    Nil(),
    Bool(bool),
    Number(LuaNumber),
    String(LuaString),
}

pub enum LuaOpCode {
    MOVE = 0x00,
    LOADK = 0x01,
    LOADBOOL = 0x02,
    LOADNIL = 0x03,
    GETUPVAL = 0x04,
    GETGLOBAL = 0x05,
    GETTABLE = 0x06,
    SETGLOBAL = 0x07,
    SETUPVAL = 0x08,
    SETTABLE = 0x09,
    NEWTABLE = 0x0a,
    SELF = 0x0b,
    ADD = 0x0c,
    SUB = 0x0d,
    MUL = 0x0e,
    DIV = 0x0f,
    MOD = 0x10,
    POW = 0x11,
    UNM = 0x12,
    NOT = 0x13,
    LEN = 0x14,
    CONCAT = 0x15,
    JMP = 0x16,
    EQ = 0x17,
    LT = 0x18,
    LE = 0x19,
    TEST = 0x1a,
    TESTSET = 0x1b,
    CALL = 0x1c,
    TAILCALL = 0x1d,
    RETURN = 0x1e,
    FORLOOP = 0x1f,
    FORPREP = 0x20,
    TFORLOOP = 0x21,
    SETLIST = 0x22,
    CLOSE = 0x23,
    CLOSURE = 0x24,
    VARARG = 0x25,
}

pub struct LuaInstruction {
    pub op_code: LuaOpCode,
    pub registers: u32,
}

pub struct LuaLocal {
    pub name: LuaString,
    pub start: LuaInteger,
    pub end: LuaInteger,
}

pub type FunctionInstructions = Vec<LuaInstruction>;
type FunctionConstants = Vec<LuaConstant>;
type FunctionPrototypes = Vec<LuaFunction>;
type FunctionLinePositions = Vec<LuaInteger>;
type FunctionLocals = Vec<LuaLocal>;
type FunctionUpValues = Vec<LuaString>;

pub struct LuaFunction {
    pub source_name: Option<LuaString>,
    pub line_defined: LuaInteger,
    pub last_line_defined: LuaInteger,
    pub up_values_count: u8,
    pub params_count: u8,
    pub is_var_arg: LuaFunctionArgType,
    pub max_stack_size: u8,

    pub instructions: FunctionInstructions,
    pub constants: FunctionConstants,
    pub prototypes: FunctionPrototypes,
    pub line_positions: FunctionLinePositions,
    pub locals: FunctionLocals,
    pub up_values: FunctionUpValues,
}
