#![feature(generic_associated_types)]

macro_rules! m {
    ($a:expr, $b:expr, $c:block) => {
        match $a {
            Lto::Fat | Lto::Thin => { $b; (); $c }
            Lto::No => { $b; () }
        }
    }
}

pub enum Lto { No, Thin, Fat }

trait ProcessCookie<'a> {
    type Output;
    fn process(&mut self, lto: Lto) -> Self::Output;
}

trait DebugProcess: ProcessCookie<'static> {
    fn debug_process(&mut self, lto: Lto) -> Self::Output where <Self as ProcessCookie<'static>>::Output: std::fmt::Display {
        let original = self.process(lto);
        println!("Processed value: {}", original);
        original
    }
}

impl<T> DebugProcess for T where T: ProcessCookie<'static> {}

impl<'a> ProcessCookie<'a> for u32 {
    type Output = u32;
    fn process(&mut self, lto: Lto) -> Self::Output {
        let mut _a = false;
        m!(lto, _a = true, { *self = 0 });
        *self
    }
}

fn f(mut cookie: u32, lto: Lto) -> u32 {
    cookie.debug_process(lto)
}

fn main() { assert_eq!(f(42, Lto::Thin), 0) }