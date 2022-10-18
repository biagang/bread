mod error;
use error::*;

mod config;
use config::Config;

mod byte_writer;
use byte_writer::ByteWriter;

mod binary;
mod hexadecimal;
mod ascii;

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
    let config = Config::new().unwrap();
    let (input, output) = config.to_io();
    match convert(input, output) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e:?}");
        }
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
        let input = [ _A, _B, _STAR, _EXCL ];
        let mut output = [0u8; 4];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(ascii::Writer::new(output.as_mut_slice()));
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
            _0, _0, _1, _0,  _1, _0, _1, _0,  _0, _0, _1, _0,  _0, _0, _0, _1,
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
        let input = [ _A, _B, _STAR, _EXCL ];
        let expected = [_6, _1, _6, _2, _2, _A, _2, _1];
        let mut output = [0u8; 8];
        let reader = Box::new(ascii::Reader::new(input.as_slice()));
        let writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(reader, writer).unwrap();
        assert_eq!(expected, output);
    }
}
