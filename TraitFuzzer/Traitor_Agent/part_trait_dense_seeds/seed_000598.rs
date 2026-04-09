#![feature(type_alias_impl_trait)]

trait Identity { fn identity(&self) -> Self; }
impl Identity for u32 { fn identity(&self) -> Self { *self } }

type Identifiable = impl Identity + PartialEq<u32> + std::fmt::Debug;

#[define_opaque(Identifiable)]
fn r#fn(r#match: u32) -> Identifiable {
    let result: u32 = r#match.identity();
    result
}

pub fn main() {
    let r#struct = 1;
    assert_eq!(1, r#struct);

    let foo = 2;
    assert_eq!(2, foo);

    let r#bar = 3;
    assert_eq!(3, r#bar);

    assert_eq!(r#fn(4), 4);

    let r#true = false;
    assert_eq!(r#true, false);
}