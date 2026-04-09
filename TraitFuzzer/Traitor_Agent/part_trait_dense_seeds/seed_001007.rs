#![feature(generic_associated_types)]

struct SelfReference {
    self_reference: *mut dyn SelfRefSetter,
}

trait SelfRefSetter {
    fn set_self_ref(&mut self);
}

trait SelfRefTrait<'a> {
    type RefType: 'a;
    fn init_self_ref(&'a mut self) where Self: SelfRefSetter + 'a;
}

impl<'a, T> SelfRefTrait<'a> for T where T: SelfRefSetter + 'a {
    type RefType = &'a mut dyn SelfRefSetter;
    fn init_self_ref(&'a mut self) {
        self.set_self_ref();
    }
}

impl SelfRefSetter for SelfReference {
    fn set_self_ref(&mut self) {
        self.self_reference = self as *mut dyn SelfRefSetter;
    }
}

fn main() {}