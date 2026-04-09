#![feature(type_alias_impl_trait)]

trait CallIt {
    type Func;
    fn call_it(&mut self, y: i32) -> i32;
}

type ClosureAlias = impl FnMut(i32, i32) -> i32;

impl<F> CallIt for F
where
    F: FnMut(i32, i32) -> i32,
{
    type Func = Self;

    fn call_it(&mut self, y: i32) -> i32 {
        self(2, y)
    }
}

#[define_opaque(ClosureAlias)]
pub fn main() {
    let mut f: ClosureAlias = |x: i32, y: i32| -> i32 { x + y };
    let z = f.call_it(3);
    println!("{}", z);
    assert_eq!(z, 5);
}