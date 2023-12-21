use super::constant::ConstantReader;
use super::instruction::InstructionsReader;
use super::line_position::FunctionLinePositionsReader;
use super::locals::FunctionLocalListReader;
use super::primitives::{read_integer, read_string};
use super::up_value::FunctionUpValuesReader;
use super::Reader;
use crate::types::{AssemblyHeader, LuaFunction, LuaFunctionArgType};
use bytes::Buf;
use bytes::Bytes;

pub struct FunctionReader;
impl Reader<LuaFunction> for FunctionReader {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<LuaFunction, String> {
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
            prototypes: FunctionReader::read_list(buf, header)?,
            line_positions: FunctionLinePositionsReader::read_list(buf, header)?,
            locals: FunctionLocalListReader::read_list(buf, header)?,
            up_values: FunctionUpValuesReader::read_list(buf, header)?,
        });
    }
}
