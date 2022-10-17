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
    fn valid(n: char) -> Option<u8> {
        if n >= '0' && n <= '9' {
            Some(n as u8 - '0' as u8)
        } else if n >= 'a' && n <= 'f' {
            Some(10u8 + (n as u8 - 'a' as u8))
        } else if n >= 'A' && n <= 'F' {
            Some(10u8 + (n as u8 - 'A' as u8))
        } else {
            None
        }
    }
    fn next_non_whitespace(&mut self) -> Option<<Bytes<R> as Iterator>::Item> {
        loop {
            let c = self.in_bytes.next()?;
            match c {
                Ok(c) => {
                    if c.is_ascii_whitespace() {
                        continue;
                    } else {
                        return Some(Ok(c));
                    }
                }
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<u8, InError>;
    fn next(&mut self) -> Option<Self::Item> {
        let msn = self.next_non_whitespace()?;
        match msn {
            Ok(msn) => {
                if let Some(msn) = Self::valid(msn as char) {
                    if let Some(lsn) = self.next_non_whitespace() {
                        match lsn {
                            Ok(lsn) => {
                                if let Some(lsn) = Self::valid(lsn as char) {
                                    Some(Ok((msn << 4) | lsn))
                                } else {
                                    Some(Err(InError::InvalidByte(lsn as char)))
                                }
                            }
                            Err(e) => Some(Err(InError::StdIO(e))),
                        }
                    } else {
                        Some(Err(InError::ShortIO {
                            bytes: 1usize,
                            expected: 2,
                        }))
                    }
                } else {
                    Some(Err(InError::InvalidByte(msn as char)))
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
        let msn = char::from_digit(((byte & 0xf0) >> 4) as u32, 16).unwrap() as u8;
        let lsn = char::from_digit((byte & 0x0f) as u32, 16).unwrap() as u8;
        util::write(&mut self.out_bytes, &[msn, lsn], 2)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::literals::*;

    #[test]
    fn read() {
        let input = [_A, _4, _1, _B.to_ascii_uppercase()];
        let mut reader = Reader::new(input.as_slice());
        assert_eq!(0xa4u8, reader.next().unwrap().unwrap());
        assert_eq!(0x1bu8, reader.next().unwrap().unwrap());
        assert!(reader.next().is_none());
    }

    #[test]
    fn write() {
        let input = 0xf4;
        let expected = [_F, _4];
        let mut output = [0u8; 2];
        let mut writer = Writer::new(output.as_mut_slice());
        writer.write(input).unwrap();
        assert_eq!(expected, output);
    }
}
