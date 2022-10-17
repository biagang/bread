use std::io::Write;
use crate::byte_writer::ByteWriter;
use crate::error::OutError;

pub struct Writer<W: Write> {
    out_bytes: W,
}

impl<W: Write> Writer<W> {
    pub fn new(out_bytes: W) -> Self {
        Writer { out_bytes }
    }
}

impl<W: Write> ByteWriter for Writer<W> {
    fn write(&mut self, byte: u8) -> Result<(), OutError> {
        let msn = char::from_digit(((byte & 0xf0) >> 4) as u32, 16).unwrap() as u8;
        let lsn = char::from_digit((byte & 0x0f) as u32, 16).unwrap() as u8;
        match self.out_bytes.write(&[msn, lsn]) {
            Ok(n) => match n {
                2 => Ok(()),
                _ => Err(OutError::ShortIO {
                    bytes: n,
                    expected: 2,
                }),
            },
            Err(e) => Err(OutError::StdIO(e)),
        }
    }
}
