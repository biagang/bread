use bread_cli as bread;
use std::fmt::Display;

use bread::ascii;
use bread::base;
use bread::binary;
use bread::byte_writer::ByteWriter;
use bread::error::*;
use bread::hexadecimal;
use bread::raw;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_parser = Mode::parse, default_value_t = Mode::Ascii, long_help = Mode::LONG_HELP)]
    /// input format
    input: Mode,

    #[arg(short, long, value_parser = Mode::parse, default_value_t = Mode::Ascii, long_help = Mode::LONG_HELP)]
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
    /// numeric base (2 to 36)
    Base(u8),
}

impl Mode {
    const LONG_HELP: &'static str = r#"Possible values:
- raw:   raw byte
- bin:   binary representation (g.e. '00001101')
- hex:   hexadecimal representation (g.e. 'a4')
- ascii: ASCII characters (g.e. '!')
- N:     base N representation (note: make sure to provide required number of digits per each byte, pad with heading 0s) "#;
    fn parse(arg: &str) -> Result<Self, String> {
        if let Ok(base) = arg.parse::<u8>() {
            if base > 1 && base < 37 {
                Ok(Mode::Base(base))
            } else {
                Err("base must be in [2,36]".to_string())
            }
        } else {
            match arg {
                "raw" | "r" => Ok(Mode::Raw),
                "bin" | "b" => Ok(Mode::Bin),
                "hex" | "h" => Ok(Mode::Hex),
                "ascii" | "a" => Ok(Mode::Ascii),
                _ => Err(
                    "allowed modes: raw, bin, hex, ascii or N where N is a numeric base in [2,36]"
                        .to_string(),
                ),
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

pub type IO = (
    Box<dyn Iterator<Item = Result<u8, InError>>>,
    Box<dyn ByteWriter>,
);

impl Config {
    pub fn new() -> Option<Self> {
        let args = Args::parse();

        Some(Config {
            reader: match args.input {
                Mode::Raw => Box::new(raw::Reader::new(std::io::stdin())),
                Mode::Bin => Box::new(binary::Reader::new(std::io::stdin())),
                Mode::Hex => Box::new(hexadecimal::Reader::new(std::io::stdin())),
                Mode::Ascii => Box::new(ascii::Reader::new(std::io::stdin())),
                Mode::Base(b) => match b {
                    2 => Box::new(binary::Reader::new(std::io::stdin())),
                    16 => Box::new(hexadecimal::Reader::new(std::io::stdin())),
                    _ => Box::new(base::Reader::new(std::io::stdin(), b)),
                },
            },
            writer: match args.output {
                Mode::Raw => Box::new(raw::Writer::new(std::io::stdout())),
                Mode::Bin => Box::new(binary::Writer::new(std::io::stdout())),
                Mode::Hex => Box::new(hexadecimal::Writer::new(std::io::stdout())),
                Mode::Ascii => Box::new(ascii::Writer::new(std::io::stdout())),
                Mode::Base(b) => match b {
                    2 => Box::new(binary::Writer::new(std::io::stdout())),
                    16 => Box::new(hexadecimal::Writer::new(std::io::stdout())),
                    _ => Box::new(base::Writer::new(std::io::stdout(), b)),
                },
            },
        })
    }
}

impl From<Config> for IO {
    fn from(config: Config) -> Self {
        (config.reader, config.writer)
    }
}
