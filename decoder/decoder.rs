use std::io::Read;
use waffle_ir::lir::module::LirModule;

use crate::DecodingError;

pub struct Decoder<R: Read> {
    inner: R,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Self { inner: reader }
    }
}
