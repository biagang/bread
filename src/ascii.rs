use crate::byte_writer::ByteWriter;
use crate::error::{InError, OutError};
use crate::util;
use std::io::{Bytes, Read, Write};

pub struct Reader<R: Read> {
    in_bytes: Bytes<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(read: R) -> Self {
        Reader {
            in_bytes: read.bytes(),
        }
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<u8, InError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.in_bytes.next()? {
            Ok(c) => {
                if c.is_ascii() {
                    Some(Ok(c))
                } else {
                    Some(Err(InError::InvalidByte(c as char))) // todo: maybe better as u8?
                }
            }
            Err(e) => Some(Err(InError::StdIO(e))),
        }
    }
}

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
        if byte.is_ascii() {
            util::write(&mut self.out_bytes, &[byte], 1)
        } else {
            Err(OutError::InvalidByte(byte))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::literals::*;

    #[test]
    fn read() {
        let input = [_B, _A, _EXCL, _STAR];
        let mut reader = Reader::new(input.as_slice());
        for b in input {
            assert_eq!(b, reader.next().unwrap().unwrap());
        }
        assert!(reader.next().is_none());
    }

    #[test]
    fn write() {
        let input = _STAR;
        let mut output = [0u8, 1];
        let mut writer = Writer::new(output.as_mut_slice());
        writer.write(input).unwrap();
        assert_eq!(input, output[0]);
    }
}
