use std::io::Error as IOError;

/// Input/output error type
#[derive(Debug)]
pub enum ErrorType<Byte> {
    /// I/O error
    ///
    /// Wraps a [std::io::Error]
    StdIO(IOError),
    /// Short read/write error
    ///
    /// Number of read/written bytes is less than expected according to the format:
    /// for example if input format is binary, number of input bytes must be a multiple of 8 (since
    /// 8 binary digits are needed to code a byte value);
    /// similarly, if output format is hexadecimal, writing a byte value must result in writing 2
    /// bytes (since 2 hexadecimal digits are needed to code a byte value)
    ShortIO { bytes: usize, expected: usize },
    /// Invalid byte read or invalid byte value to write
    ///
    /// According to expected input format, a char read from the input can be invalid: f.e. in case
    /// of binary format any character other  than '0' or '1' is invalid.
    /// Depending on output format, not all possible byte values can be represented; f.e. in case
    /// of ASCII format only byte values less than 128 are valid.
    InvalidByte(Byte),
}

/// Input error
pub type InError = ErrorType<char>;
/// Output error
pub type OutError = ErrorType<u8>;

/// The error type returned by [convert].
///
/// [convert]: crate::convert
#[derive(Debug)]
pub enum Error {
    /// Input error
    ///
    /// Error originated in reading or parsing the input
    In(InError),
    /// Output error
    ///
    /// Error originated in parsing or writing the output
    Out(OutError),
}
