struct Foo(String);

impl Drop for Foo {
    fn drop(&mut self) {}
}

trait Resettable {
    fn reset(&mut self, new_value: String);
}

impl Resettable for Foo {
    fn reset(&mut self, new_value: String) {
        self.0 = new_value;
    }
}

fn test_inline_replacement() {
    let _s = ();
    let mut f = Foo(String::from("foo"));
    f.reset(String::from("bar"));
}

fn test_outline_replacement() {
    let _s = String::from("foo");
    let mut f = Foo(_s);
    f.reset(String::from("bar"));
}

fn main() {
    test_inline_replacement();
    test_outline_replacement();
}