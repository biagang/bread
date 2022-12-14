use crate::byte_writer::ByteWriter;
use crate::error::{InError, OutError};
use crate::util;
use std::io::{Bytes, Read, Write};

/// An iterator over Result<u8,[InError]>
///
/// Reads bytes from the input stream in hexadecimal base format, that is a multiple of 2 characters in the ranges ('0','9'), ('a','f') or ('A', 'F') are allowed (and any number of whitespace characters that will be skipped)
///
/// [InError]: crate::error::InError
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
        if ('0'..='9').contains(&n) {
            Some(n as u8 - b'0')
        } else if ('a'..='f').contains(&n) {
            Some(10u8 + (n as u8 - b'a'))
        } else if ('A'..='F').contains(&n) {
            Some(10u8 + (n as u8 - b'A'))
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

/// Writes bytes to the output stream in the hexadecimal format
///
/// Produced characters are in the ranges ('0', '9') and ('a', 'f')
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

#[cfg(all(test, feature = "benchmark"))]
mod benchs {
    extern crate test;
    use super::*;
    use crate::util::literals::*;

    #[bench]
    fn read(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static INPUT: [u8; N] = [_F; N];
        b.iter(|| {
            let reader = Reader::new(INPUT.as_slice());
            let _ = reader.collect::<Vec<Result<u8, InError>>>();
        });
    }

    #[bench]
    fn write(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static mut OUTPUT: [u8; N] = [_0; N];
        b.iter(|| unsafe {
            let mut writer = Writer::new(OUTPUT.as_mut_slice());
            for _ in 0..N / 2 {
                writer.write(255u8).unwrap();
            }
        });
    }
}
