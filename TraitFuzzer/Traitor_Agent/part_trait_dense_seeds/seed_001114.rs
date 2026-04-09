#![feature(generic_associated_types)]
#![allow(dead_code)]

trait RangeHandler {
    type Item<'a> where Self: 'a;

    fn handle(&self, value: Self::Item<'_>);
}

trait RangeHandlerExt: RangeHandler {
    fn safe_handle(&self, value: Self::Item<'_>) -> bool {
        self.handle(value);
        true
    }
}

impl<T: RangeHandler> RangeHandlerExt for T {}

impl RangeHandler for i8 {
    type Item<'a> = &'a i8;

    fn handle(&self, value: Self::Item<'_>) {
        if *value >= -128 && *value <= 127 {
            foo(*value);
        }
    }
}

fn foo(_: i8) {}

fn main() {}