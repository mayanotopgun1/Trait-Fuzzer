#![feature(type_alias_impl_trait)]

const fn make() -> (i32, i32, *const i32) {
    const V: i32 = 123;
    &V as *const i32;
    (0, 0, &V)
}

trait ArrayMaker {
    type Out;
    fn make() -> Self::Out;
}

impl ArrayMaker for () {
    type Out = (i32, i32, *const i32);
    fn make() -> Self::Out {
        make()
    }
}

fn main() {
    let arr: [<() as ArrayMaker>::Out; 32] = [<() as ArrayMaker>::make(); 32];
    println!("{}", arr[0].0);
}