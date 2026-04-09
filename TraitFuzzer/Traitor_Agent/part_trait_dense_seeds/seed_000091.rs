trait EqCheck {
    fn check_eq(&self, other: i32);
}

trait EqCheckExt: EqCheck {}

impl<T> EqCheckExt for T where T: EqCheck {}

impl EqCheck for i32 {
    fn check_eq(&self, other: i32) {
        assert_eq!(*self, other);
    }
}

fn main() {
    let mut x = Box::new(1);
    match *x {
        y => {
            x = Box::new(2);
            let _tmp = 1;
            y.check_eq(1);
        }
    }
}