trait Get {
    type Value;
    fn get(&self) -> &<Self as Get>::Value;
}

struct Struct {
    x: isize,
}

impl Get for Struct {
    type Value = isize;
    fn get(&self) -> &isize {
        &self.x
    }
}

trait Grab {
    type U;
    fn grab(&self) -> &<Self as Grab>::U;
}

trait ExtendedGrab: Grab {
    fn double_grab(&self) -> (&<Self as Grab>::U, &<Self as Grab>::U) {
        let u = self.grab();
        (u, u)
    }
}

impl<T: Grab> ExtendedGrab for T {}

impl<T: Get> Grab for T {
    type U = <T as Get>::Value;
    fn grab(&self) -> &<T as Get>::Value {
        self.get()
    }
}

trait AdditionalTrait {
    fn additional_method(&self);
}

impl<T: Get> AdditionalTrait for T where <T as Get>::Value: std::fmt::Debug {
    fn additional_method(&self) {
        println!("Additional method called on {:?}", *self.get());
    }
}

fn main() {
    let s = Struct {
        x: 100,
    };
    assert_eq!(*s.double_grab().0, 100);
    s.additional_method();
}