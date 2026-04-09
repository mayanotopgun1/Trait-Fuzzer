#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[derive(Debug)]
pub struct Ident { name: usize }

macro_rules! int3 { () => ( { } ) }

fn ident_new() -> Ident {
    int3!();
    Ident {name: 0x6789ABCD }
}

pub trait Bomb { fn boom(&self, _: Ident); }
pub struct S;
impl Bomb for S { fn boom(&self, _: Ident) { } }

pub trait BombExt: Bomb {
    fn light_fuse(&self) {
        int3!();
        let f = || {
            int3!();
            self.boom(ident_new());
        };
        f();
    }
}
impl<T: Bomb + ?Sized> BombExt for T {}

pub fn main() {
    let b = Box::new(S) as Box<dyn Bomb>;
    b.light_fuse();
}