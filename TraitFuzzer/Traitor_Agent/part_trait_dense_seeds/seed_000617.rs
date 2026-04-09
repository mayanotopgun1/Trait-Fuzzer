#![crate_type = "lib"]

trait VecProducer {
    type Out<'a>;
    fn produce_vec<'a>() -> Self::Out<'a>;
}

impl VecProducer for () {
    type Out<'a> = &'a [u8];
    fn produce_vec<'a>() -> Self::Out<'a> { &[] }
}

pub fn vec() -> &'static [u8] {
    <()>::produce_vec()
}