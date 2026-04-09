#![feature(type_alias_impl_trait)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

trait ColorProcessor {
    fn process(&self) -> isize;
}

#[allow(dead_code)]
enum color {
    rgb(isize, isize, isize),
    rgba(isize, isize, isize, isize),
    hsl(isize, isize, isize),
}

impl ColorProcessor for color {
    fn process(&self) -> isize {
        let mut x: isize;
        match self {
            color::rgb(r, _, _) => { x = *r; }
            color::rgba(_, _, _, a) => { x = *a; }
            color::hsl(_, s, _) => { x = *s; }
        }
        return x;
    }
}

pub fn main() {
    let gray: color = color::rgb(127, 127, 127);
    let clear: color = color::rgba(50, 150, 250, 0);
    let red: color = color::hsl(0, 255, 255);
    assert_eq!(gray.process(), 127);
    assert_eq!(clear.process(), 0);
    assert_eq!(red.process(), 255);
}