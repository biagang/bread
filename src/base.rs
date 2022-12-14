use crate::byte_writer::ByteWriter;
use crate::error::{InError, OutError};
use crate::util;
use crate::util::literals::*;
use std::io::{Bytes, Read, Write};

struct Base {
    base: u8,
    digits_per_byte: u8,
}

impl Base {
    fn new(base: u8) -> Self {
        Base {
            base,
            digits_per_byte: 256f32.log(base.into()).ceil() as u8,
        }
    }
    fn valid(&self, n: char) -> Option<u8> {
        let digit = if ('0'..='9').contains(&n) {
            n as u8 - b'0'
        } else if ('a'..='z').contains(&n) {
            10u8 + (n as u8 - b'a')
        } else if ('A'..='Z').contains(&n) {
            10u8 + (n as u8 - b'A')
        } else {
            36u8
        };
        if digit < self.base {
            Some(digit)
        } else {
            None
        }
    }
    fn to_char(&self, d: u8) -> Option<u8> {
        if d < self.base {
            if d < 10 {
                Some(_0 + d)
            } else {
                Some(_A + (d - 10))
            }
        } else {
            None
        }
    }
}

/// An iterator over Result<u8,[InError]>
///
/// Reads bytes from the input stream in the expected numeric base format, that means allowed
/// characters depend on the particular numeric base (in any case in the ranges ('0', '9'), ('a', 'z') or ('A', 'Z'); any whitespace character is allowed and skipped)
///
/// [InError]: crate::error::InError
pub struct Reader<R: Read> {
    in_bytes: Bytes<R>,
    base: Base,
}

impl<R: Read> Reader<R> {
    pub fn new(read: R, base: u8) -> Self {
        Reader {
            in_bytes: read.bytes(),
            base: Base::new(base),
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
        let msi = (self.base.digits_per_byte - 1) as i8;
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
                            expected: self.base.digits_per_byte as usize,
                        }))
                    }
                }
                Some(in_byte) => match in_byte {
                    Ok(in_byte) => {
                        let in_char = in_byte as char;
                        if let Some(digit) = self.base.valid(in_char) {
                            value += digit * self.base.base.pow(i as u32);
                        } else {
                            return Some(Err(InError::InvalidByte(in_char)));
                        }
                    }
                    Err(e) => {
                        return Some(Err(InError::StdIO(e)));
                    }
                },
            }
            i -= 1;
        }
        Some(Ok(value))
    }
}

/// Writes bytes to the output stream in the provided numeric base format
///
/// Produced characters depend on the particular numeric base, in any case in the range ('0', '9') and ('a','z')
pub struct Writer<W: Write> {
    out_bytes: W,
    base: Base,
}

impl<W: Write> Writer<W> {
    pub fn new(out_bytes: W, base: u8) -> Self {
        Writer {
            out_bytes,
            base: Base::new(base),
        }
    }
}

impl<W: Write> ByteWriter for Writer<W> {
    fn write(&mut self, byte: u8) -> Result<(), OutError> {
        let mut byte = byte;
        let mut string = vec![_0; self.base.digits_per_byte as usize];
        let mut i = string.len() - 1;
        loop {
            let digit = byte % self.base.base;
            byte /= self.base.base;
            string[i] = self.base.to_char(digit).unwrap();
            if byte != 0 {
                i -= 1;
            } else {
                break;
            }
        }
        util::write(
            &mut self.out_bytes,
            string.as_slice(),
            self.base.digits_per_byte as usize,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGITS: [u8; 36] = [
        _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _A, _B, _C, _D, _E, _F, _G, _H, _I, _J, _K, _L, _M,
        _N, _O, _P, _Q, _R, _S, _T, _U, _V, _W, _X, _Y, _Z,
    ];

    fn required_digits(base: u8) -> u8 {
        match base {
            2 => 8,
            3 => 6,
            4..=6 => 4,
            7..=15 => 3,
            16..=36 => 2,
            _ => panic!("invalid base {base}"),
        }
    }

    #[test]
    fn base_digits_per_byte() {
        for base in 2..37 {
            assert_eq!(Base::new(base).digits_per_byte, required_digits(base));
        }
    }

    #[test]
    fn base_valid_digits() {
        for b in 2..37 {
            let base = Base::new(b);
            for value in 0..DIGITS.len() {
                let digit = DIGITS[value];
                let in_chars = [digit as char, (digit as char).to_ascii_uppercase()];
                for in_char in in_chars {
                    let result = base.valid(in_char);
                    if value < b as usize {
                        assert_eq!(Some(value as u8), result);
                    } else {
                        assert_eq!(None, result, "base {b}, value: {value}");
                    }
                }
            }
            assert_eq!(None, base.valid('*'));
            assert_eq!(None, base.valid('!'));
        }
    }

    #[test]
    fn base_to_char() {
        for b in 2..37 {
            let base = Base::new(b);
            for d in 0..b {
                assert_eq!(Some(DIGITS[d as usize]), base.to_char(d));
            }
            assert_eq!(None, base.to_char(base.base));
            assert_eq!(None, base.to_char(125));
        }
    }

    #[test]
    fn b2_read() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let mut reader = Reader::new(input.as_slice(), 2);
        assert_eq!(0b01001010u8, reader.next().unwrap().unwrap());
        assert_eq!(0b01011111u8, reader.next().unwrap().unwrap());
        assert!(reader.next().is_none());
    }

    #[test]
    fn b2_write() {
        let input = 0b10110100u8;
        let expected = [_1, _0, _1, _1, _0, _1, _0, _0];
        let mut output = [0u8; 8];
        let mut writer = Writer::new(output.as_mut_slice(), 2);
        writer.write(input).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn b8_write0() {
        let input = 0;
        let expected = [_0, _0, _0];
        let mut output = [0u8; 3];
        let mut writer = Writer::new(output.as_mut_slice(), 8);
        writer.write(input).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn b36_read() {
        let mut input = [_0; DIGITS.len() * 2];
        for i in 0..input.len() {
            if i % 2 == 1 {
                input[i] = DIGITS[i / 2];
            }
        }
        let mut reader = Reader::new(input.as_slice(), 36);
        for i in 0u8..36u8 {
            let got = reader.next();
            if got.is_none() {
                panic!("reader.next() returned None for {i}");
            }
            let got = got.unwrap();
            if got.is_err() {
                panic!("{got:?} returned for {i}");
            }
            assert_eq!(i, got.unwrap());
        }
        assert!(reader.next().is_none());
    }

    #[test]
    fn b36_write() {
        let mut input = [0u8; 36];
        let mut expected = [_0; 72];
        for i in 0..DIGITS.len() {
            input[i] = i as u8;
            expected[2 * i + 1] = DIGITS[i];
        }
        let mut output = [0u8; 72];
        let mut writer = Writer::new(output.as_mut_slice(), 36);
        for b in input {
            writer.write(b).unwrap();
        }
        assert_eq!(expected, output);
    }
}

#[cfg(all(test, feature = "benchmark"))]
mod benchs {
    extern crate test;
    use super::*;

    #[bench]
    fn b2_read(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static INPUT: [u8; N] = [_1; N];
        b.iter(|| {
            let reader = Reader::new(INPUT.as_slice(), 2);
            let _ = reader.collect::<Vec<Result<u8, InError>>>();
        });
    }

    #[bench]
    fn b2_write(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static mut OUTPUT: [u8; N] = [_0; N];
        b.iter(|| unsafe {
            let mut writer = Writer::new(OUTPUT.as_mut_slice(), 2);
            for _ in 0..N / 8 {
                writer.write(255u8).unwrap();
            }
        });
    }

    #[bench]
    fn b16_read(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static INPUT: [u8; N] = [_F; N];
        b.iter(|| {
            let reader = Reader::new(INPUT.as_slice(), 16);
            let _ = reader.collect::<Vec<Result<u8, InError>>>();
        });
    }

    #[bench]
    fn b16_write(b: &mut test::Bencher) {
        const N: usize = 1024 * 1024;
        static mut OUTPUT: [u8; N] = [_0; N];
        b.iter(|| unsafe {
            let mut writer = Writer::new(OUTPUT.as_mut_slice(), 16);
            for _ in 0..N / 2 {
                writer.write(255u8).unwrap();
            }
        });
    }
}
