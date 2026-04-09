type A = Box<dyn (Fn(u8) -> u8) + 'static + Send + Sync>;

trait Call {
    fn call(&self, x: u8) -> u8;
}

impl<F> Call for F
where
    F: Fn(u8) -> u8,
{
    fn call(&self, x: u8) -> u8 {
        (self)(x)
    }
}

fn main() {}