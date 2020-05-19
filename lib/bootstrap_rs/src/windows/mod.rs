#![allow(bad_style)]

pub mod file;
pub mod input;
pub mod window;

pub trait ToCU16Str {
    fn to_c_u16(&self) -> Vec<u16>;
}

impl<'a> ToCU16Str for &'a str {
    fn to_c_u16(&self) -> Vec<u16> {
        self.encode_utf16().chain(Some(0).into_iter()).collect()
    }
}
