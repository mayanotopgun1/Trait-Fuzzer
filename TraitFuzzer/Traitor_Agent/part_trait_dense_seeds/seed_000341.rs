trait CallIt {
    fn call_it(&mut self, y: i32) -> i32;
}

impl<F> CallIt for F
where
    F: FnMut(i32, i32) -> i32,
{
    fn call_it(&mut self, y: i32) -> i32 {
        self(2, y)
    }
}

pub fn main() {
    let mut f = |x: i32, y: i32| -> i32 { x + y };
    let z = f.call_it(3);
    println!("{}", z);
    assert_eq!(z, 5);
}