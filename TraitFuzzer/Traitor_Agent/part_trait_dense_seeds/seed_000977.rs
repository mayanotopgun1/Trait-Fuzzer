trait HasDefault<const N: usize> {
    fn has_default();
}

impl<const N: usize> HasDefault<N> for () where [(); N]: Default {
    fn has_default() {}
}

fn main() {
    <() as HasDefault<1>>::has_default();
}