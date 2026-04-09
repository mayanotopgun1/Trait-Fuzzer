trait Producer<T> {
    fn produce() -> Result<&'static str, T>;
}

impl<T> Producer<T> for () {
    fn produce() -> Result<&'static str, T> {
        Ok("22")
    }
}

fn main() {
    let x: usize = <() as Producer<()>>::produce()
        .and_then(|x| x.parse::<usize>().map_err(|_| ()))
        .unwrap_or_else(|_| panic!());

    assert_eq!(x, 22);
}