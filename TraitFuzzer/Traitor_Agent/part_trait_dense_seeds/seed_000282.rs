#![allow(non_camel_case_types)]
#![allow(dead_code)]

trait FooExt {
    fn foo_ext(&self, c: Vec<isize>) -> impl core::fmt::Debug;
}

impl FooExt for () {
    fn foo_ext(&self, c: Vec<isize>) -> impl core::fmt::Debug {
        let a: isize = 5;
        let mut b: Vec<isize> = Vec::new();

        match t::none::<isize> {
            t::some::<isize>(_) => {
                for _i in &c {
                    println!("{}", a);
                    let a = 17;
                    b.push(a);
                }
            }
            _ => {}
        }

        b
    }
}

enum t<T> {
    none,
    some(T),
}

pub fn main() {
    let x = 10;
    let x = x + 20;
    assert_eq!(x, 30);
    let result = ().foo_ext(Vec::new());
    println!("{:?}", result);
}