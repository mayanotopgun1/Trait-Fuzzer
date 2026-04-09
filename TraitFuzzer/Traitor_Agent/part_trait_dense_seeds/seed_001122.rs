#![allow(unused_variables)]
pub trait Parameters { type SelfRef; }

struct RP<'a> { _marker: std::marker::PhantomData<&'a ()> }
struct BP;

impl<'a> Parameters for RP<'a> { type SelfRef = &'a X<RP<'a>>; }
impl Parameters for BP { type SelfRef = Box<X<BP>>; }

pub struct Y;
pub enum X<P: Parameters> {
    Nothing,
    SameAgain(P::SelfRef, Y)
}

trait Constructible<P: Parameters>: Sized {
    fn new() -> P::SelfRef;
}

impl<'a> Constructible<RP<'a>> for X<RP<'a>> {
    fn new() -> &'a X<RP<'a>> { 
        Box::leak(Box::new(X::Nothing)) 
    }
}

impl Constructible<BP> for X<BP> {
    fn new() -> Box<X<BP>> { 
        Box::new(X::Nothing) 
    }
}

fn main() {
    let bnil: Box<X<BP>> = X::<BP>::new();
    let bx: Box<X<BP>> = Box::new(X::SameAgain(bnil, Y));
    let rnil: X<RP> = X::Nothing;
    let rx: X<RP> = X::SameAgain(&rnil, Y);
}