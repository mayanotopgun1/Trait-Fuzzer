#![feature(const_default)]
#![feature(const_trait_impl)]
#![feature(default_field_values)]
#![feature(derive_const)]

const trait DefaultFields {
    fn default_fields() -> Self;
}

#[derive(PartialEq, Eq, Debug)]
#[derive_const(Default)]
struct S {
    r: Option<String> = <Option<_> as Default>::default(),
    s: String = String::default(),
    o: Option<String> = Option::<String>::default(),
    p: std::marker::PhantomData<()> = std::marker::PhantomData::default(),
    q: Option<String> = <Option<String> as Default>::default(),
    t: Option<String> = Option::default(),
    v: Option<String> = const { Option::default() },
}

impl const DefaultFields for S {
    fn default_fields() -> Self {
        S { .. }
    }
}

const _: S = S::default_fields();
const _: S = const { S::default_fields() };
const _: S = S::default();
const _: S = const { S::default() };

fn main() {
    let s = S::default_fields();
    assert_eq!(s.r, None);
    assert_eq!(&s.s, "");
    assert_eq!(s.o, None);
    assert_eq!(s.p, std::marker::PhantomData);
    assert_eq!(s.q, None);
    assert_eq!(s.t, None);
    assert_eq!(s.v, None);
    assert_eq!(s, S::default());
}