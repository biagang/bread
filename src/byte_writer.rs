use crate::error::*;

/// A trait for objects which are byte-oriented sinks.
///
/// This trait looks similar to [Write] trait; major difference is for the write method to return [Error] type
///
/// [Write]: std::io::Write
/// [Error]: crate::error::OutError
pub trait ByteWriter {
    fn write(&mut self, byte: u8) -> Result<(), OutError>;
}
