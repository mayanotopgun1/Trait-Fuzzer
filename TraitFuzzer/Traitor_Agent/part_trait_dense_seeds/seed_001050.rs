trait ModG {
    fn g() -> isize;
}

mod m {
    pub fn g() -> isize { 720 }
}

impl ModG for () {
    fn g() -> isize { m::g() }
}

fn f() -> isize {
    <()>::g()
}

pub fn main() {
    assert_eq!(f(), 720);
}