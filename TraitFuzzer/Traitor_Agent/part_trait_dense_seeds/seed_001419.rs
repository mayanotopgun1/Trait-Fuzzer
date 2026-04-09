trait Function {
    fn call(&self) -> isize;
}

impl Function for () {
    fn call(&self) -> isize {
        3
    }
}

fn f() -> impl Function {
    ()
}

fn main() {
    assert_eq!(f().call(), 3);
}