trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

trait OptionMirror: Mirror {
    fn none_value() -> Self::Assoc where Self::Assoc: Default;
}
impl<T> OptionMirror for Option<T> {
    fn none_value() -> Self::Assoc where Self::Assoc: Default {
        None
    }
}

type Foo<T> = <Option<T> as Mirror>::Assoc;

fn main() {
    let x: Foo<i32> = Option::<i32>::none_value();
}