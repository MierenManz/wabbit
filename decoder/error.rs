use leb128::read::Error as VarintError;
use std::io;
pub enum DecodingError {
    VarintOverflow,
    IoError(io::ErrorKind),
    UnexpectedSection(u8),
}

impl From<VarintError> for DecodingError {
    fn from(value: VarintError) -> Self {
        match value {
            VarintError::IoError(v) => Self::IoError(v.kind()),
            VarintError::Overflow => Self::VarintOverflow,
        }
    }
}

impl From<io::Error> for DecodingError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.kind())
    }
}
