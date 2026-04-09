trait RefMaker<'a> { fn make_ref(&self) -> &'a (); }

impl<'a> RefMaker<'a> for () {
    fn make_ref(&self) -> &'a () {
        &()
    }
}

fn creash<'a>() {
    let x: Box<dyn for<'b> RefMaker<'b>> = Box::new(());
    let _ = x.make_ref();
}

fn produce<'a>() {
    move || {
        let x: Box<dyn for<'b> RefMaker<'b>> = Box::new(());
        let _ = x.make_ref();
    };
}

fn main() {}