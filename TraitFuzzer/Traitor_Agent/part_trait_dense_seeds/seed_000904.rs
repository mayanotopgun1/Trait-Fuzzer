trait FooTrait {
    fn foo(&mut self, n: i32);
}

impl FooTrait for i32 {
    fn foo(&mut self, mut n: i32) {
        if false {
            n = 0i32;
        }

        if n > 0i32 {
            let _ = 1i32 / n;
        }
    }
}

fn main() {
    let mut x = 10i32;
    x.foo(10);
}