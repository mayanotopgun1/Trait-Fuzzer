trait Trait {
    fn foo(&self) {}
}

trait StaticTrait: Trait {}

impl<T: Trait + 'static> StaticTrait for T {}

fn bar<'a>(a: *mut *mut (dyn Trait + 'a)) -> *mut *mut (dyn StaticTrait + 'static) {
    a as _
}

fn main() {}