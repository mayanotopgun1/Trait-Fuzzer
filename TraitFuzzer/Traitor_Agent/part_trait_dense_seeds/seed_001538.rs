macro_rules! m {
    () => { #[cfg(false)] fn f() {} }
}

trait T {}
trait TM: T { m!(); }
impl<S> TM for S where S: T {}

impl T for () {}

fn main() {}