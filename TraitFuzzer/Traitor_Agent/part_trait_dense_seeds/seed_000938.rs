trait Trait<'a, 'b> {}
impl Trait<'_, '_> for () {}

trait TraitPass<'a: 'b, 'b: 'a> { fn pass(&self) -> impl Trait<'a, 'b>; }
impl<'a: 'b, 'b: 'a> TraitPass<'a, 'b> for () {
    fn pass(&self) -> impl Trait<'a, 'b> {
        (|| {})()
    }
}

struct Foo<'a>(&'a ());
impl<'a> Foo<'a> {
    fn bar<'b: 'a>(&'b self) -> impl Trait<'a, 'b> {
        let _: &'a &'b &'a ();
    }
}

fn main() {}