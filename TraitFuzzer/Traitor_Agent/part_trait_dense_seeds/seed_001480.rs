trait Boo {
    fn dummy(&self) {}
}

trait BooExt: Boo {}

impl<T> BooExt for T where T: Boo {}

impl Boo for [i8; 1] {}
impl Boo for [i8; 2] {}
impl Boo for [i8; 3] {}
impl Boo for [i8; 4] {}

pub fn foo(box_1: impl Fn() -> Box<[i8; 1]>,
           box_2: impl Fn() -> Box<[i8; 2]>,
           box_3: impl Fn() -> Box<[i8; 3]>,
           box_4: impl Fn() -> Box<[i8; 4]>) {
    println!("Hello World 1");
    let _: Box<dyn BooExt> = match 3 {
        1 => box_1(),
        2 => box_2(),
        3 => box_3(),
        _ => box_4(),
    };
    println!("Hello World 2");
}

pub fn main() {
    foo(
        || Box::new([1; 1]),
        || Box::new([1; 2]),
        || Box::new([1; 3]),
        || Box::new([1; 4]),
    );
}