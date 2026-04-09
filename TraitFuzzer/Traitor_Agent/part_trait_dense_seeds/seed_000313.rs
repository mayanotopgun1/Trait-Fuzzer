struct SelfReference {
    self_reference: *mut SelfReference,
}

trait SelfRefSetter {
    fn set_self_ref(&mut self);
}

impl SelfRefSetter for SelfReference {
    fn set_self_ref(&mut self) {
        self.self_reference = self;
    }
}

fn main() {}