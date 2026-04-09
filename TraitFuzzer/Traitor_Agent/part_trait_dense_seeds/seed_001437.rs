#![allow(unused)]

struct Struct {
    x: i32,
    y: i32,
    s: String,
}

trait Modify {
    fn modify_x(&mut self, inc: i32);
    fn modify_y(&mut self, inc: i32);
    fn set_s(&mut self, new_s: String);
}

impl Modify for Struct {
    fn modify_x(&mut self, inc: i32) {
        self.x += inc;
    }

    fn modify_y(&mut self, inc: i32) {
        self.y += inc;
    }

    fn set_s(&mut self, new_s: String) {
        self.s = new_s;
    }
}

fn main() {
    let mut s = Struct { x: 10, y: 10, s: String::new() };

    let mut c = || {
        s.modify_x(10);
        s.modify_y(42);
        s.set_s(String::from("new"));
    };
}