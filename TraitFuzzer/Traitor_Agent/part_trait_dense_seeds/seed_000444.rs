trait Identity {
    fn identity(x: Self) -> Self where Self: Sized;
}

impl Identity for u32 {
    fn identity(x: Self) -> Self where Self: Sized {
        x
    }
}

fn main() {
    assert_eq!(<u32 as Identity>::identity(18), 18);
}