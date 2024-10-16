use leb128::read::Error as VarintError;
pub enum DecodingError {
    VarintOverflow,
    IoError(std::io::ErrorKind),
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