#![feature(return_position_impl_trait_in_trait)]

trait OptionUtils<T> {
    fn double_option(self) -> impl Iterator<Item = Option<T>>;
}

impl<T> OptionUtils<T> for Option<T> {
    fn double_option(self) -> impl Iterator<Item = Option<T>> {
        self.into_iter().map(|v| Some(v))
    }
}

fn main() {
    let x: Vec<_> = Some(Some(Some(1))).double_option().collect();

    match &x[0] {
        Some(Some(v)) => (),
        _ => (),
    }
}