use std::fmt::Display;

use crate::ascii;
use crate::base;
use crate::binary;
use crate::byte_writer::ByteWriter;
use crate::error::*;
use crate::hexadecimal;
use crate::raw;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = Mode::parse, default_value_t = Mode::Ascii)]
    /// input format
    input: Mode,

    #[arg(short, long, value_parser = Mode::parse, default_value_t = Mode::Ascii)]
    /// output format
    output: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Mode {
    /// raw byte
    Raw,
    /// binary representation (g.e. '00001101')
    Bin,
    /// hexadecimal representation (g.e. 'a4')
    Hex,
    /// ASCII characters (g.e. '!')
    Ascii,
    /// numeric base (2 to 16)
    Base(u8),
}

impl Mode {
    fn parse(arg: &str) -> Result<Self, String> {
        if let Ok(base) = arg.parse::<u8>() {
            if base > 1 && base < 17 {
                Ok(Mode::Base(base))
            } else {
                Err(format!("base must be in [2,16]"))
            }
        } else {
            match arg {
                "raw" | "r" => Ok(Mode::Raw),
                "bin" | "b" => Ok(Mode::Bin),
                "hex" | "h" => Ok(Mode::Hex),
                "ascii" | "a" => Ok(Mode::Ascii),
                _ => Err(format!(
                    "allowed modes: raw, bin, hex, ascii or X where X is a numeric base in [2,16]"
                )),
            }
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Raw => "raw".to_string(),
                Mode::Bin => "bin".to_string(),
                Mode::Hex => "hex".to_string(),
                Mode::Ascii => "ascii".to_string(),
                Mode::Base(b) => format!("base {b}"),
            }
        )
    }
}

pub struct Config {
    reader: Box<dyn Iterator<Item = Result<u8, InError>>>,
    writer: Box<dyn ByteWriter>,
}

impl Config {
    pub fn new() -> Option<Self> {
        let args = Args::parse();
        Some(Config {
            reader: match args.input {
                Mode::Raw => Box::new(raw::Reader::new(std::io::stdin())),
                Mode::Bin => Box::new(binary::Reader::new(std::io::stdin())),
                Mode::Hex => Box::new(hexadecimal::Reader::new(std::io::stdin())),
                Mode::Ascii => Box::new(ascii::Reader::new(std::io::stdin())),
                Mode::Base(b) => Box::new(base::Reader::new(std::io::stdin(), b)),
            },
            writer: match args.output {
                Mode::Raw => Box::new(raw::Writer::new(std::io::stdout())),
                Mode::Bin => Box::new(binary::Writer::new(std::io::stdout())),
                Mode::Hex => Box::new(hexadecimal::Writer::new(std::io::stdout())),
                Mode::Ascii => Box::new(ascii::Writer::new(std::io::stdout())),
                Mode::Base(_) => unimplemented!(),
            },
        })
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
