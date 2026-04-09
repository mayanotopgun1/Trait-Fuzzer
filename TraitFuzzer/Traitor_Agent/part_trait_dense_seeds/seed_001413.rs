struct D;

trait Tr {
    type It;
    fn foo(&self) -> Option<Self::It>;
}

impl<'a> Tr for &'a D {
    type It = ();
    fn foo(&self) -> Option<()> { None }
}

trait RunExt: Tr {
    fn run<F>(&self, f: F)
        where F: Fn(<Self as Tr>::It),
              Self: Sized,
    {
        while let Some(i) = self.foo() {
            f(i);
        }
    }
}

impl<'a> RunExt for &'a D {}

fn main() {
    (&D).run(|_| {});
}