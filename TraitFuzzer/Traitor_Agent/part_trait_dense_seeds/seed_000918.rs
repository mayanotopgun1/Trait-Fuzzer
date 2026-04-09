trait Foo { fn foo(&mut self, n: i32); }
trait FooExt: Foo {
    fn foo_safe(&mut self, mut n: i32) -> Result<(), DivideByZeroError> {
        if false {
            n = 0i32;
        }

        if n > 0i32 {
            Ok(())
        } else {
            Err(DivideByZeroError)
        }
    }
}
impl<T: Foo> FooExt for T {}
struct DivideByZeroError;

impl Foo for i32 {
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
    let mut x = 10;
    match x.foo_safe(x) {
        Ok(_) => x.foo(x),
        Err(DivideByZeroError) => println!("Division by zero error"),
    }
}