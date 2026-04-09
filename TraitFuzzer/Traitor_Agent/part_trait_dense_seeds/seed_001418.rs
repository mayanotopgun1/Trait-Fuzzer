trait Function {
    fn call(&self) -> isize;
}

impl Function for () {
    fn call(&self) -> isize {
        3
    }
}

fn f() -> isize {
    let func: &dyn Function = &();
    func.call()
}

fn main() {
    assert_eq!(f(), 3);
}