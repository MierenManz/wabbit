use crate::Decode;
use crate::DecodingError;
use leb128::read;
use std::io::Read;

impl Decode for u64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        Ok(read::unsigned(reader)?)
    }
}

impl Decode for u32 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        u64::decode(reader)?
            .try_into()
            .map_err(|_| DecodingError::VarintOverflow)
    }
}

impl Decode for u16 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        u64::decode(reader)?
            .try_into()
            .map_err(|_| DecodingError::VarintOverflow)
    }
}

impl Decode for u8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodingError> {
        u64::decode(reader)?
            .try_into()
            .map_err(|_| DecodingError::VarintOverflow)
    }
}
