#![feature(generic_associated_types)]

trait MyTrait<'a> {
    type Output;
}

trait MyTraitExt<'a, T>: MyTrait<'a>
where
    Self::Output: 'a,
{
    fn foo(&self) -> &'a () {
        self.bar()
    }

    fn bar(&self) -> &'a () {
        &()
    }
}

impl<'a, T> MyTraitExt<'a, T> for T where T: MyTrait<'a>, <T as MyTrait<'a>>::Output: 'a {}

fn main() {}