pub mod function;
pub mod header;

mod primitives;
mod instruction;
mod line_position;
mod locals;
mod up_value;
mod constant;

use crate::types::{AssemblyHeader, LuaInteger};
use bytes::Bytes;

pub(crate) trait Reader<T> {
    fn read(buf: &mut Bytes, header: &AssemblyHeader) -> Result<T, String>
    where
        Self: Sized;
    fn read_list(buf: &mut Bytes, header: &AssemblyHeader) -> Result<Vec<T>, String>
    where
        Self: Sized,
    {
        let list_size = primitives::read_integer(buf, header)?;
        let real_size: usize = match list_size {
            LuaInteger::I32(x) => x.try_into().unwrap(),
        };

        if real_size == 0 {
            return Ok(vec![]);
        }

        let mut result = Vec::with_capacity(real_size);
        for _ in 0..real_size {
            let item = Self::read(buf, header)?;
            result.push(item);
        }

        return Ok(result);
    }
}
