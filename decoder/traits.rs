use crate::error::DecodingError;
use std::io::Read;

pub trait Decode<R>
where
    R: Read,
    Self: Sized,
{
    fn decode(reader: &mut R) -> Result<Self, DecodingError>;
}
