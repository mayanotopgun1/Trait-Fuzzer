#![allow(dead_code)]

trait XTrait {
    fn get_b(&self) -> u8;
    fn get_c(&self) -> bool;
    fn get_d(&self) -> u8;
    fn get_e(&self) -> u16;
    fn get_f(&self) -> u8;
    fn get_g(&self) -> u8;
}

impl<T> XTrait for X<T> {
    fn get_b(&self) -> u8 {
        self.b
    }
    
    fn get_c(&self) -> bool {
        self.c
    }
    
    fn get_d(&self) -> u8 {
        self.d
    }
    
    fn get_e(&self) -> u16 {
        self.e
    }
    
    fn get_f(&self) -> u8 {
        self.f
    }
    
    fn get_g(&self) -> u8 {
        self.g
    }
}

struct X<T> {
    a: T,
    b: u8,
    c: bool,
    d: u8,
    e: u16,
    f: u8,
    g: u8
}

pub fn main() {
    let x: X<isize> = X {
        a: 12345678,
        b: 9,
        c: true,
        d: 10,
        e: 11,
        f: 12,
        g: 13
    };
    bar(x);
}

fn bar<T>(x: X<T>) {
    assert_eq!(x.get_b(), 9);
    assert_eq!(x.get_c(), true);
    assert_eq!(x.get_d(), 10);
    assert_eq!(x.get_e(), 11);
    assert_eq!(x.get_f(), 12);
    assert_eq!(x.get_g(), 13);
}