#![feature(autodiff)]

trait Func {
    fn apply(&self, x: f64) -> f64;
}

macro_rules! demo {
    () => {
        #[std::autodiff::autodiff_reverse(fd, Active, Active)]
        fn f(x: f64) -> f64 {
            x * x
        }
    };
}
demo!();

fn main() {
    dbg!(f(2.0f64));
}