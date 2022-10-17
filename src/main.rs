mod error;
use error::*;

mod config;
use config::Config;

mod byte_writer;
use byte_writer::ByteWriter;

mod binary;
mod hexadecimal;


fn convert<>(
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
        Ok(()) => {},
        Err(e) => {eprintln!("{e:?}");},
    }
}

const _0: u8 = '0' as u8;
const _1: u8 = '1' as u8;
const _2: u8 = '2' as u8;
const _3: u8 = '3' as u8;
const _4: u8 = '4' as u8;
const _5: u8 = '5' as u8;
const _6: u8 = '6' as u8;
const _7: u8 = '7' as u8;
const _8: u8 = '8' as u8;
const _9: u8 = '9' as u8;
const _A: u8 = 'a' as u8;
const _B: u8 = 'b' as u8;
const _C: u8 = 'c' as u8;
const _D: u8 = 'd' as u8;
const _E: u8 = 'e' as u8;
const _F: u8 = 'f' as u8;


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
