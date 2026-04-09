#![feature(return_position_impl_trait_in_trait)]

trait PrintFormat {
    fn print(&self);
}

trait ExtendedPrint: PrintFormat {
    fn print_twice(&self) -> impl Fn() {
        let self_ref = self;
        move || {
            self_ref.print();
            self_ref.print();
        }
    }
}

impl<T> ExtendedPrint for T where T: PrintFormat {}

impl PrintFormat for f64 {
    fn print(&self) {
        println!("{}", self);
    }
}

fn main() {
    let values = [0E+10, 0e+10, 00e+10, 00E+10];
    for &value in &values {
        value.print_twice()();
    }
}