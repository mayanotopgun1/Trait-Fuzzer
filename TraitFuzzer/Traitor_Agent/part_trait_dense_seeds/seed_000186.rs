struct Thing;
struct Dummy;

trait DummyTrait {
    type DummyType;
}
impl DummyTrait for Dummy {
    type DummyType = Thing;
}
type AlsoThing = <Dummy as DummyTrait>::DummyType;

trait SomeTrait {
    type Item;
}
trait SomeTraitExt: SomeTrait<Item = AlsoThing> {
    fn is_thing(&self) -> bool {
        true
    }
}
impl<T> SomeTraitExt for T where T: SomeTrait<Item = AlsoThing> {}

type TraitObject = dyn SomeTrait<Item = AlsoThing>;
type AlsoTraitObject = dyn SomeTrait<Item = Thing>;

trait Supertrait {
    type Foo;
}

trait Subtrait: Supertrait<Foo = TraitObject> {}

trait HasOutput<A: ?Sized> {
    type Output;
}

fn foo<F>() -> F::Output
where
    F: HasOutput<dyn Subtrait<Foo = AlsoTraitObject>>,
{
    todo!()
}

fn main() {}