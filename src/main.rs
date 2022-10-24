mod error;
use error::*;

mod config;
use config::Config;

mod byte_writer;
use byte_writer::ByteWriter;

mod ascii;
mod base;
mod binary;
mod hexadecimal;
mod raw;

mod util;

fn convert(
    istream: Box<dyn Iterator<Item = Result<u8, InError>> + '_>,
    mut ostream: Box<dyn ByteWriter + '_>,
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

fn main() {
    let (input, output) = Config::new().unwrap().to_io();
    if let Err(e) = convert(input, output) {
        eprintln!("{e:?}");
    }
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
        convert(reader, writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn hex2hex() {
        let input = [_A, _7, _B.to_ascii_uppercase(), _3];
        let mut output = [0u8; 4];
        let reader = Box::new(hexadecimal::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(input.to_ascii_lowercase(), output);
    }

    #[test]
    fn ascii2ascii() {
        let input = [_A, _B, _STAR, _EXCL];
        let mut output = [0u8; 4];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(ascii::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn raw2raw() {
        let input = [10u8, 128u8, 255u8, 4u8];
        let mut output = [0u8; 4];
        let reader = Box::new(raw::Reader::new(input.as_slice()));
        let writer = Box::new(raw::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn bin2hex() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let expected = [_4, _A, _5, _F];
        let mut output = [0u8; 4];
        let reader = Box::new(binary::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
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
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn ascii2hex() {
        let input = [_A, _B, _STAR, _EXCL];
        let expected = [_6, _1, _6, _2, _2, _A, _2, _1];
        let mut output = [0u8; 8];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn raw2hex() {
        let input = [0xfa, 0x4b];
        let expected = [_F, _A, _4, _B];
        let mut output = [0u8; 4];
        let reader = Box::new(raw::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
    }

    #[test]
    fn ascii2raw() {
        let input = [_0, _A, _2, _EXCL, _STAR];
        let mut output = [0u8; 5];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(raw::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn b16_hex() {
        let input = [_1, _F];
        let mut output = [0u8; 2];
        let reader = Box::new(base::Reader::new(input.as_slice(), 16));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn b10_hex() {
        let input = [_0, _1, _2];
        let mut output = [0u8; 2];
        let expected = [_0, _C];
        let reader = Box::new(base::Reader::new(input.as_slice(), 10));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
    }
}
