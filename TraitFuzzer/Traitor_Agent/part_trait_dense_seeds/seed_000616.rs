#![crate_type = "lib"]

trait VecProducer {
    type Out;
    fn produce_vec() -> Self::Out;
}

impl VecProducer for () {
    type Out = Vec<u8>;
    fn produce_vec() -> Vec<u8> { vec![] }
}

pub fn vec() -> Vec<u8> {
    <()>::produce_vec()
}