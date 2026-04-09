#![allow(dead_code)]
#![allow(non_camel_case_types)]

struct Font<'a> {
    fontbuf: &'a Vec<u8>,
}

trait BufAccess<'a> {
    fn buf(&self) -> &'a Vec<u8>;
}

impl<'a> BufAccess<'a> for Font<'a> {
    fn buf(&self) -> &'a Vec<u8> {
        self.fontbuf
    }
}

fn create_font(fontbuf: &Vec<u8>) -> Box<dyn BufAccess<'_> + '_> {
    Box::new(Font { fontbuf })
}

pub fn main() {}