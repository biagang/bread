mod error;
use error::*;

mod config;
use config::Config;

mod byte_writer;
use byte_writer::ByteWriter;

mod binary;
mod hexadecimal;

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
        let bin_reader = Box::new(binary::Reader::new(input.as_slice()));
        let bin_writer = Box::new(binary::Writer::new(output.as_mut_slice()));
        convert(bin_reader, bin_writer).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn hex2hex() {
        let input = [_A, _7, _B.to_ascii_uppercase(), _3];
        let mut output = [0u8; 4];
        let hex_reader = Box::new(hexadecimal::Reader::new(input.as_slice()));
        let hex_writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(hex_reader, hex_writer).unwrap();
        assert_eq!(input.to_ascii_lowercase(), output);
    }

    #[test]
    fn bin2hex() {
        let input = [
            _0, _1, _0, _0, _1, _0, _1, _0, _0, _1, _0, _1, _1, _1, _1, _1,
        ];
        let expected = [_4, _A, _5, _F];
        let mut output = [0u8; 4];
        let bin_reader = Box::new(binary::Reader::new(input.as_slice()));
        let hex_writer = Box::new(hexadecimal::Writer::new(output.as_mut_slice()));
        convert(bin_reader, hex_writer).unwrap();
        assert_eq!(expected, output);
    }
}
