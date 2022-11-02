use crate::error::OutError;
use std::io::Write;

pub mod literals {
    pub const _0: u8 = b'0';
    pub const _1: u8 = b'1';
    pub const _2: u8 = b'2';
    pub const _3: u8 = b'3';
    pub const _4: u8 = b'4';
    pub const _5: u8 = b'5';
    pub const _6: u8 = b'6';
    pub const _7: u8 = b'7';
    pub const _8: u8 = b'8';
    pub const _9: u8 = b'9';
    pub const _A: u8 = b'a';
    pub const _B: u8 = b'b';
    pub const _C: u8 = b'c';
    pub const _D: u8 = b'd';
    pub const _E: u8 = b'e';
    pub const _F: u8 = b'f';
    pub const _G: u8 = b'g';
    pub const _H: u8 = b'h';
    pub const _I: u8 = b'i';
    pub const _J: u8 = b'j';
    pub const _K: u8 = b'k';
    pub const _L: u8 = b'l';
    pub const _M: u8 = b'm';
    pub const _N: u8 = b'n';
    pub const _O: u8 = b'o';
    pub const _P: u8 = b'p';
    pub const _Q: u8 = b'q';
    pub const _R: u8 = b'r';
    pub const _S: u8 = b's';
    pub const _T: u8 = b't';
    pub const _U: u8 = b'u';
    pub const _V: u8 = b'v';
    pub const _W: u8 = b'w';
    pub const _X: u8 = b'x';
    pub const _Y: u8 = b'y';
    pub const _Z: u8 = b'z';
    pub const _EXCL: u8 = b'!';
    pub const _STAR: u8 = b'*';
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
