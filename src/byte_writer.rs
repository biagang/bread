use crate::error::*;

pub trait ByteWriter {
    fn write(&mut self, byte: u8) -> Result<(), OutError>;
}
