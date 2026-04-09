#![crate_type = "lib"]

trait SliceExt<'input> {
    fn new(slice: &'input [u8], extra: u32) -> Self;
}

impl<'input> SliceExt<'input> for ExtraSlice<'input> {
    fn new(slice: &'input [u8], extra: u32) -> Self {
        ExtraSlice { slice, extra }
    }
}

pub struct ExtraSlice<'input> {
    slice: &'input [u8],
    extra: u32,
}

#[no_mangle]
pub fn extra(s: &[u8]) {
    let slice = ExtraSlice::new(s, s.len() as u32);
}

struct Zst;

trait SliceExtZst<'input> {
    fn new(slice: &'input [u8], extra: Zst) -> Self;
}

impl<'input> SliceExtZst<'input> for ZstSlice<'input> {
    fn new(slice: &'input [u8], extra: Zst) -> Self {
        ZstSlice { slice, extra }
    }
}

pub struct ZstSlice<'input> {
    slice: &'input [u8],
    extra: Zst,
}

#[no_mangle]
pub fn zst(s: &[u8]) {
    let slice = ZstSlice::new(s, Zst);
}