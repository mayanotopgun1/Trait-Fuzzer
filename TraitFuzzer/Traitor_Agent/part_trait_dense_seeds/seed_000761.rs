trait IfTrue {
    fn if_true(&self) -> impl Into<i32>;
}

impl IfTrue for bool {
    fn if_true(&self) -> impl Into<i32> {
        if *self {
            10
        } else {
            panic!()
        }
    }
}

pub fn main() {
    let _x = true.if_true().into();
}