#[macro_export]
macro_rules! separator {
    () => { "/" };
}

#[macro_export]
macro_rules! concat_separator {
    ( $e:literal, $($other:literal),+ ) => {
        concat!($e, $crate::separator!(), $crate::concat_separator!($($other),+))
    };
    ( $e:literal ) => {
        $e
    }
}

trait ConcatSeparator {
    fn concat_separator(self) -> String;
}

impl<T> ConcatSeparator for T
where
    T: IntoIterator,
    T::Item: AsRef<str>,
{
    fn concat_separator(self) -> String {
        self.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(separator!())
    }
}

fn main() {
    let args = vec![2.to_string(), 3.to_string(), 4.to_string()];
    println!("{}", args.concat_separator());
}