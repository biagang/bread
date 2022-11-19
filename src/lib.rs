#![cfg_attr(feature = "benchmark", feature(test))]

pub mod error;
use error::*;

pub mod byte_writer;
use byte_writer::ByteWriter;

pub mod ascii;
pub mod base;
pub mod binary;
pub mod hexadecimal;
pub mod raw;

mod util;

#[derive(Clone)]
pub struct Format {
    pub item_separator: String,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            item_separator: "".to_string(),
        }
    }
}

/// Converts byte input stream format to byte output stream format
///
/// Iterates on bytes in istream and [writes] them to ostream.
///
/// [writes]: crate::byte_writer::ByteWriter::write
///
/// # Errors
///
/// see [ErrorType] for error details.
///
/// [ErrorType]: crate::error::ErrorType
///
/// # Examples
///
/// binary to hexadecimal conversion
/// ```
/// use bread_cli::*;
/// const _0: u8 = '0' as u8;
/// const _1: u8 = '1' as u8;
/// const _4: u8 = '4' as u8;
/// const _5: u8 = '5' as u8;
/// const _A: u8 = 'a' as u8;
/// const _F: u8 = 'f' as u8;
///
/// let input = [ _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1, ];
/// let mut output = [0u8; 4];
/// let reader = Box::new(binary::Reader::new(input.as_slice()));
/// let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
/// convert(reader, writer, None).unwrap();
/// assert_eq!([_4, _A, _5, _F], output);
/// ```
///
pub fn convert(
    istream: Box<dyn Iterator<Item = Result<u8, InError>> + '_>,
    mut ostream: Box<dyn ByteWriter + '_>,
    _out_fmt: Option<Format>,
) -> Result<(), Error> {
    for input in istream {
        match input {
            Ok(input) => {
                if let Err(out_error) = ostream.write(input) {
                    return Err(Error::Out(out_error));
                }
            }
            Err(in_error) => {
                return Err(Error::In(in_error));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::literals::*;

    #[test]
    fn bin2bin() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let mut output = [0u8; 16];
        let reader = Box::new(binary::Reader::new(input.as_slice()));
        let writer = Box::new(binary::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn hex2hex() {
        let input = [_A, _7, _B.to_ascii_uppercase(), _3];
        let mut output = [0u8; 4];
        let reader = Box::new(hexadecimal::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input.to_ascii_lowercase(), output);
    }

    #[test]
    fn ascii2ascii() {
        let input = [_A, _B, _STAR, _EXCL];
        let mut output = [0u8; 4];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(ascii::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn raw2raw() {
        let input = [10u8, 128u8, 255u8, 4u8];
        let mut output = [0u8; 4];
        let reader = Box::new(raw::Reader::new(input.as_slice()));
        let writer = Box::new(raw::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn bin2hex() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let mut output = [0u8; 4];
        let reader = Box::new(binary::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!([_4, _A, _5, _F], output);
    }

    #[test]
    fn bin2ascii() {
        let input = [
            _0, _0, _1, _0, _1, _0, _1, _0, _0, _0, _1, _0, _0, _0, _0, _1,
        ];
        let expected = [_STAR, _EXCL];
        let mut output = [0u8; 2];
        let reader = Box::new(binary::Reader::new(input.as_slice()));
        let writer = Box::new(ascii::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn ascii2hex() {
        let input = [_A, _B, _STAR, _EXCL];
        let expected = [_6, _1, _6, _2, _2, _A, _2, _1];
        let mut output = [0u8; 8];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn raw2hex() {
        let input = [0xfa, 0x4b];
        let expected = [_F, _A, _4, _B];
        let mut output = [0u8; 4];
        let reader = Box::new(raw::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn ascii2raw() {
        let input = [_0, _A, _2, _EXCL, _STAR];
        let mut output = [0u8; 5];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(raw::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn b16_hex() {
        let input = [_1, _F];
        let mut output = [0u8; 2];
        let reader = Box::new(base::Reader::new(input.as_slice(), 16));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn b10_hex() {
        let input = [_0, _1, _2];
        let mut output = [0u8; 2];
        let expected = [_0, _C];
        let reader = Box::new(base::Reader::new(input.as_slice(), 10));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer, None).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn b10_to_b16() {
        let input = [_0, _1, _6, _2, _5, _4];
        let mut output = [0u8; 4];
        let expected = [_1, _0, _F, _E];
        let reader = Box::new(base::Reader::new(input.as_slice(), 10));
        let writer = Box::new(base::Writer::new(output.as_mut_slice(), 16));
        convert(reader, writer, None).unwrap();
        assert_eq!(expected, output);
    }
}
