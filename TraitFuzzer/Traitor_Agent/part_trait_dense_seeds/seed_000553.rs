#![feature(generic_associated_types)]

trait PanicableExt: Panicable {
    fn trigger_panic_ext(&self) -> Self::Panic<'_> where Self: Sized {
        self.trigger_panic()
    }
}

impl<T: Panicable> PanicableExt for T {}

trait Panicable {
    type Panic<'a> where Self: 'a;
    fn trigger_panic(&self) -> Self::Panic<'_>;
}

struct P;

impl Panicable for P {
    type Panic<'a> = &'a str;
    fn trigger_panic(&self) -> Self::Panic<'_> {
        panic!("test")
    }
}

fn main() {
    let p = P;
    let _ = p.trigger_panic_ext();
}