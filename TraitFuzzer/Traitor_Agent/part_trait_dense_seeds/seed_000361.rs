unsafe extern "C" {
    static TEST1: i32;
    fn test1(i: i32);
}

trait TestTrait {
    unsafe fn test(&self);
}

impl TestTrait for () {
    unsafe fn test(&self) {
        test1(TEST1);
    }
}

fn test2() {
    unsafe {
        let _ = <() as TestTrait>::test(&());
    }
}

fn main() {}