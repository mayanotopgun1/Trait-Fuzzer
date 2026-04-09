trait RefMaker<'a> { fn make_ref(&self) -> &'a (); }

impl<'a> RefMaker<'a> for () {
    fn make_ref(&self) -> &'a () {
        &()
    }
}

fn creash<'a>() {
    let _ = <() as RefMaker<'a>>::make_ref(&());
}

fn produce<'a>() {
    move || {
        let _ = <() as RefMaker<'a>>::make_ref(&());
    };
}

fn main() {}