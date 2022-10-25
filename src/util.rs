use crate::error::OutError;
use std::io::Write;

pub mod literals {
    pub const _0: u8 = '0' as u8;
    pub const _1: u8 = '1' as u8;
    pub const _2: u8 = '2' as u8;
    pub const _3: u8 = '3' as u8;
    pub const _4: u8 = '4' as u8;
    pub const _5: u8 = '5' as u8;
    pub const _6: u8 = '6' as u8;
    pub const _7: u8 = '7' as u8;
    pub const _8: u8 = '8' as u8;
    pub const _9: u8 = '9' as u8;
    pub const _A: u8 = 'a' as u8;
    pub const _B: u8 = 'b' as u8;
    pub const _C: u8 = 'c' as u8;
    pub const _D: u8 = 'd' as u8;
    pub const _E: u8 = 'e' as u8;
    pub const _F: u8 = 'f' as u8;
    pub const _EXCL: u8 = '!' as u8;
    pub const _STAR: u8 = '*' as u8;
}

pub fn write<W: Write>(
    out_bytes: &mut W,
    bytes: &[u8],
    expected_write: usize,
) -> Result<(), OutError> {
    match out_bytes.write(bytes) {
        Ok(n) => {
            if n == expected_write {
                Ok(())
            } else {
                Err(OutError::ShortIO {
                    bytes: n,
                    expected: expected_write,
                })
            }
        }
        Err(e) => Err(OutError::StdIO(e)),
    }
}
