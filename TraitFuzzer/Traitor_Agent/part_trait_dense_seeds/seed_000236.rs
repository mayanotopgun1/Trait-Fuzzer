trait OptionUtils<T> {
    fn double_option(self) -> Option<Option<T>>;
}

impl<T> OptionUtils<T> for Option<T> {
    fn double_option(self) -> Option<Option<T>> {
        self.map(|v| Some(v))
    }
}

fn main() {
    let x = Some(Some(Some(1))).double_option();

    match x {
        Some::<Option<_>>(Some(Some(v))) => (),
        _ => (),
    }
}