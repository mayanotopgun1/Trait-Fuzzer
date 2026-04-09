#![allow(unused)]

struct S;

trait FuncTrait<U> {
    fn func(&self) -> U;
}

impl<U> FuncTrait<U> for S {
    fn func(&self) -> U {
        todo!()
    }
}

trait ExtendedFuncTrait<U>: FuncTrait<U> where U: Copy {
    fn extended_func(&self) -> U {
        self.func()
    }
}

impl<T, U> ExtendedFuncTrait<U> for T where T: FuncTrait<U>, U: Copy {}

fn dont_crash<'a, U>() -> U
where
    S: ExtendedFuncTrait<U>,
    U: Copy,
{
    let s = S;
    s.extended_func()
}

fn main() {}