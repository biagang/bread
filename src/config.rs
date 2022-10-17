use clap::{Parser, ValueEnum};
use crate::byte_writer::ByteWriter;
use crate::error::*;
use crate::binary;
use crate::hexadecimal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::Ascii)]
    /// input format
    input: Mode,

    #[arg(short, long, value_enum, default_value_t = Mode::Ascii)]
    /// output format
    output: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum Mode {
    /// raw byte
    Raw,
    /// binary representation (g.e. '00001101')
    Bin,
    /// hexadecimal representation (g.e. 'a4')
    Hex,
    /// ASCII characters (g.e. '!')
    Ascii,
}

pub struct Config {
    reader: Box<dyn Iterator<Item = Result<u8, InError>>>,
    writer: Box<dyn ByteWriter>,
}

impl Config {
    pub fn new() -> Option<Self> {
        let args = Args::parse();

        let reader = match args.input {
            Mode::Raw => unimplemented!(),
            Mode::Bin => Box::new(binary::Reader::new(std::io::stdin())),
            Mode::Hex => unimplemented!(),
            Mode::Ascii => unimplemented!(),
        };
        let writer = match args.output {
            Mode::Raw => unimplemented!(),
            Mode::Bin => unimplemented!(),
            Mode::Hex => Box::new(hexadecimal::Writer::new(std::io::stdout())),
            Mode::Ascii => unimplemented!(),
        };
        Some(Config{reader, writer})
    }

    pub fn to_io(
        self,
    ) -> (
        Box<dyn Iterator<Item = Result<u8, InError>>>,
        Box<dyn ByteWriter>,
    ) {
        (self.reader, self.writer)
    }
}
