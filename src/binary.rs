use std::io::{Bytes, Read};
use crate::error::InError;

pub struct Reader<R: Read> {
    in_bytes: Bytes<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(read: R) -> Self {
        Reader {
            in_bytes: read.bytes(),
        }
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<u8, InError>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut value = 0u8;
        let mut i = 7i8;
        while i >= 0 {
            let in_byte = self.in_bytes.next();
            match in_byte {
                None => {
                    return if i == 7 {
                        None
                    } else {
                        Some(Err(InError::ShortIO {
                            bytes: 7 - i as usize,
                            expected: 8,
                        }))
                    }
                }
                Some(in_byte) => match in_byte {
                    Ok(in_byte) => {
                        let in_byte = in_byte as char;
                        match in_byte {
                            '0' => {}
                            '1' => {
                                value = value | (1 << i);
                            }
                            _ => {
                                if in_byte.is_ascii_whitespace() {
                                    continue;
                                } else {
                                    return Some(Err(InError::InvalidByte(in_byte)));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        return Some(Err(InError::StdIO(e)));
                    }
                },
            }
            i = i - 1;
        }
        Some(Ok(value))
    }
}
