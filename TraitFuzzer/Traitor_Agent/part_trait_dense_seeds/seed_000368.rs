const fn make() -> (i32, i32, *const i32) {
    const V: i32 = 123;
    &V as *const i32;
    (0, 0, &V)
}

trait ArrayMaker {
    fn make() -> (i32, i32, *const i32);
}

impl ArrayMaker for () {
    fn make() -> (i32, i32, *const i32) {
        make()
    }
}

fn main() {
    let arr = [<() as ArrayMaker>::make(); 32];
    println!("{}", arr[0].0);
}