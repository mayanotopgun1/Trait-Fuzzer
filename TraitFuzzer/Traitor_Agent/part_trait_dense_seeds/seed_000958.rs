trait Trait {
    type Rigid: Elaborate<Assoc = String> + Into<String> + Default;
}

trait Elaborate: Into<Self::Assoc> {
    type Assoc;
}

trait ElaborateDefault: Elaborate + Default {
    fn elaborate_default() -> Self {
        Default::default()
    }
}

impl<T: Elaborate + Default> ElaborateDefault for T {}

fn test<T: Trait>() {
    let rigid: T::Rigid = <T::Rigid as ElaborateDefault>::elaborate_default();
    drop(rigid.into());
}

fn main() {}