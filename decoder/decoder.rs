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
        // self.reader.bytes()
        Ok(())
    }

    pub fn decode_into(mut self, module: &mut LirModule) -> Result<(), DecodingError> {

        let mut i = 0;
        loop {
            if i >= 13 {
                break;
            }

            let section_id: u8 = varint_read(&mut self.reader)?
                .try_into()
                .map_err(|_| DecodingError::VarintOverflow)?;
    
            // ID is lower than expected. This is wrong
            if section_id <= i && section_id != 0 {
                return Err(DecodingError::UnexpectedSection(section_id))
            }
    
            match i {
                1 => self.decode_type_section(module)?,
                // 2 => 
                // 3 => self.decode_fn_section(module)?,
                4 => {},
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
