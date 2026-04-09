trait HasDefault<const N: usize> {
    fn has_default(&self);
}

impl<const N: usize> HasDefault<N> for () where [(); N]: Default {
    fn has_default(&self) {}
}

fn main() {
    let obj = Box::new(()) as Box<dyn HasDefault<1>>;
    obj.has_default();
}