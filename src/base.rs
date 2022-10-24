use crate::byte_writer::ByteWriter;
use crate::error::{InError, OutError};
use crate::util;
use std::io::{Bytes, Read, Write};

pub struct Reader<R: Read> {
    in_bytes: Bytes<R>,
    base: u8,
    digits_per_byte: u8,
}

impl<R: Read> Reader<R> {
    pub fn new(read: R, base: u8) -> Self {
        Reader {
            in_bytes: read.bytes(),
            base,
            digits_per_byte: 256f32.log(base.into()).ceil() as u8,
        }
    }

    fn valid(&self, n: char) -> Option<u8> {
        let digit = if n >= '0' && n <= '9' {
            n as u8 - '0' as u8
        } else if n >= 'a' && n <= 'f' {
            10u8 + (n as u8 - 'a' as u8)
        } else if n >= 'A' && n <= 'F' {
            10u8 + (n as u8 - 'A' as u8)
        } else {
            16u8
        };
        if digit < self.base {
            Some(digit)
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
        let mut value = 0u8;
        let msi = (self.digits_per_byte - 1) as i8;
        let mut i = msi as i8;
        while i >= 0 {
            let in_byte = self.next_non_whitespace();
            match in_byte {
                None => {
                    return if i == msi {
                        None
                    } else {
                        Some(Err(InError::ShortIO {
                            bytes: (msi - i) as usize,
                            expected: self.digits_per_byte as usize,
                        }))
                    }
                }
                Some(in_byte) => match in_byte {
                    Ok(in_byte) => {
                        let in_char = in_byte as char;
                        if let Some(digit) = self.valid(in_char) {
                            value = value + (digit * self.base.pow(i as u32));
                        } else {
                            return Some(Err(InError::InvalidByte(in_char)));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::literals::*;

    fn required_digits(base: u8) -> u8 {
        match base {
            2 => 8,
            3 => 6,
            4..=6 => 4,
            7..=15 => 3,
            16 => 2,
            _ => panic!("invalid base {base}"),
        }
    }

    #[test]
    fn digits_per_byte() {
        let input = [_0];
        for base in 2..17 {
            assert_eq!(
                Reader::new(input.as_slice(), base).digits_per_byte,
                required_digits(base)
            );
        }
    }

    #[test]
    fn valid_digits() {
        let digits = [
            _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F,
        ];
        let input = [_0];
        for base in 2..17 {
            let reader = Reader::new(input.as_slice(), base);
            for value in 0..digits.len() {
                let digit = digits[value];
                let in_chars = [digit as char, (digit as char).to_ascii_uppercase()];
                for in_char in in_chars {
                    let result = reader.valid(in_char);
                    if value < base as usize {
                        assert_eq!(Some(value as u8), result);
                    } else {
                        assert_eq!(None, result);
                    }
                }
            }
            assert_eq!(None, reader.valid('p'));
            assert_eq!(None, reader.valid('G'));
        }
    }

    #[test]
    fn base_2_read() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let mut reader = Reader::new(input.as_slice(), 2);
        assert_eq!(0b01001010u8, reader.next().unwrap().unwrap());
        assert_eq!(0b01011111u8, reader.next().unwrap().unwrap());
        assert!(reader.next().is_none());
    }
}
