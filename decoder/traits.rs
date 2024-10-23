use waffle_ir::{common::FuncType, lir::module::LirModule};

use crate::error::DecodingError;
use std::io::Read;

pub trait Decode
where
    Self: Sized,
{
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError>;
}

impl<D: Decode> Decode for Vec<D> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        let count: u32 = Decode::decode(reader)?;
        let mut v = Vec::with_capacity(count as usize);

        for _ in 0..count {
            v.push(Decode::decode(reader)?);
        }

        Ok(v)
    }
}
