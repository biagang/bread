use std::io::Error as IOError;

#[derive(Debug)]
pub enum ErrorType<Byte> {
    StdIO(IOError),
    ShortIO { bytes: usize, expected: usize },
    InvalidByte(Byte),
}

pub type InError = ErrorType<char>;
pub type OutError = ErrorType<u8>;

#[derive(Debug)]
pub enum Error {
    In(InError),
    Out(OutError),
}
