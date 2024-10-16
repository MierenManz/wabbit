use std::io::Read;
use waffle_ir::lir::module::LirModule;
use leb128::read::unsigned as varint_read;

use crate::DecodingError;

pub struct Decoder<R: Read> {
    reader: R,
}

impl<R: Read> Decoder<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn decode_type_section(&mut self, module: &mut LirModule) -> Result<(), DecodingError> {
        // for
        Ok(())
    }

    pub fn decode_into(mut self, module: &mut LirModule) -> Result<(), DecodingError> {

        for mut i in 0..13u8 {
            let section_id: u8 = varint_read(&mut self.reader)?
                .try_into()
                .map_err(|_| DecodingError::VarintOverflow)?;

            // ID is lower than expected. This is wrong
            if section_id <= i && section_id != 0 {
                return Err(DecodingError::UnexpectedSection(section_id))
            }

            match i {
                1 => {},
                _ => {}
            }

            i = section_id;
        }
        Ok(())
    }

    pub fn decode(self) -> Result<LirModule, DecodingError> {
        let mut module = LirModule::new();
        self.decode_into(&mut module)?;

        Ok(module)
    }
}
