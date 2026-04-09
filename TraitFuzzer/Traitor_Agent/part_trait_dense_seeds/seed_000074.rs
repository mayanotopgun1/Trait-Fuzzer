#[derive(Debug)]
#[allow(dead_code)]
struct Pair<T, V> (T, V);

trait Sayable {
    fn say(&self);
}

trait SayableExt: Sayable {
    fn say_twice(&self) {
        self.say();
        self.say();
    }
}

impl<'a, T: Sayable> SayableExt for T {}

impl<'a> Sayable for Pair<&'a str, isize> {
    fn say(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let result = &Pair("shane", 1);
    result.say_twice();
}