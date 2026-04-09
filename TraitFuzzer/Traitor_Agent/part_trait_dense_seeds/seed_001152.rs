trait Trait {
    type Assoc: Into<u32>;
}
impl<T: Into<u32>> Trait for T {
    type Assoc = T;
}
trait AliasBoundProjection: Trait {
    fn project(x: Self::Assoc) -> u32 {
        x.into()
    }
}
impl<T: Trait> AliasBoundProjection for T {}

fn prefer_alias_bound_projection<T: AliasBoundProjection>(x: T::Assoc) {
    let x = <T as AliasBoundProjection>::project(x);
    assert_eq!(std::mem::size_of_val(&x), 4);
}

fn impl_trait() -> impl Into<u32> {
    0u16
}

fn main() {
    let x = impl_trait().into();
    assert_eq!(std::mem::size_of_val(&x), 4);

    prefer_alias_bound_projection::<u16>(1);
}