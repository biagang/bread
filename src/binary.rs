use crate::byte_writer::ByteWriter;
use crate::error::{InError, OutError};
use crate::util;
use crate::util::literals::*;
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
        let mut value = 0u8;
        let mut i = 7i8;
        while i >= 0 {
            let in_byte = self.in_bytes.next();
            match in_byte {
                None => {
                    return if i == 7 {
                        None
                    } else {
                        Some(Err(InError::ShortIO {
                            bytes: 7 - i as usize,
                            expected: 8,
                        }))
                    }
                }
                Some(in_byte) => match in_byte {
                    Ok(in_byte) => {
                        let in_byte = in_byte as char;
                        match in_byte {
                            '0' => {}
                            '1' => {
                                value = value | (1 << i);
                            }
                            _ => {
                                if in_byte.is_ascii_whitespace() {
                                    continue;
                                } else {
                                    return Some(Err(InError::InvalidByte(in_byte)));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        return Some(Err(InError::StdIO(e)));
                    }
                },
            }
            i = i - 1;
        }
        Some(Ok(value))
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
        let mut bit_string = [_0; 8];
        for i in (0..8).rev() {
            if (byte & (1 << i)) != 0 {
                bit_string[7 - i] = _1;
            }
        }
        util::write(&mut self.out_bytes, bit_string.as_slice(), 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let mut reader = Reader::new(input.as_slice());
        assert_eq!(0b01001010u8, reader.next().unwrap().unwrap());
        assert_eq!(0b01011111u8, reader.next().unwrap().unwrap());
        assert!(reader.next().is_none());
    }

    #[test]
    fn write() {
        let input = 0b10110100u8;
        let expected = [_1, _0, _1, _1, _0, _1, _0, _0];
        let mut output = [0u8; 8];
        let mut writer = Writer::new(output.as_mut_slice());
        writer.write(input).unwrap();
        assert_eq!(expected, output);
    }
}
