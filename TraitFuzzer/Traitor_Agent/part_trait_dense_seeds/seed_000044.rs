#![feature(never_patterns)]
#![allow(incomplete_features)]

enum Void {}

trait VoidTrait { fn by_value(self); }
impl VoidTrait for Void { 
    fn by_value(self) {
        move || {
            let ! = self;
        };
    }
}

fn main() {}