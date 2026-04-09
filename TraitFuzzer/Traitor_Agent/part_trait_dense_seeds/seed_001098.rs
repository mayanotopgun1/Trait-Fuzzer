trait Cat {
    fn meow(&self) -> bool;
    fn scratch(&self) -> bool { self.purr() }
    fn purr(&self) -> bool { true }
}

trait CatEx: Cat {
    fn can_meow_twice(&self) -> bool { self.meow() && self.meow() }
}

impl<T: Cat> CatEx for T {}

impl Cat for isize {
    fn meow(&self) -> bool {
        self.scratch()
    }
}

pub fn main() {
    assert!(5.can_meow_twice());
}