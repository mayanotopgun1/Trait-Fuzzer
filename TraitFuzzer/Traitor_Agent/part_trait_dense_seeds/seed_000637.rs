trait Trait {
    type Assoc;
}

impl Trait for () {
    type Assoc = ();
}

trait UnitTrait {
    fn unit(self) -> impl Into<<() as Trait>::Assoc>;
}

impl UnitTrait for () {
    fn unit(self) -> impl Into<<() as Trait>::Assoc> {}
}

pub fn ice() {
    UnitTrait::unit(()).into();
}

fn main() {}