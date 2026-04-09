#![feature(specialization)]

trait Producer<T> {
    fn produce() -> Result<&'static str, T>;
}

default impl<T: Default> Producer<T> for () {
    fn produce() -> Result<&'static str, T> {
        Err(T::default())
    }
}

impl Producer<()> for () {
    fn produce() -> Result<&'static str, ()> {
        Ok("22")
    }
}

fn main() {
    let x: usize = <() as Producer<()>>::produce()
        .and_then(|x| x.parse::<usize>().map_err(|_| ()))
        .unwrap_or_else(|_| panic!());

    assert_eq!(x, 22);
}