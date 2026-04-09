trait BugTrait {
    fn bug<T>() -> impl Iterator<Item = [(); { || Some(false); 4 }]> where T: ?Sized;
}

impl BugTrait for () {
    fn bug<T>() -> impl Iterator<Item = [(); { || Some(false); 4 }]> where T: ?Sized {
        std::iter::empty()
    }
}

fn main() {}