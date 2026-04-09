#![allow(dead_code)]
#![feature(impl_trait_in_assoc_type)]

static DATA: &'static [&'static str] = &["my string"];

trait DataRef {
    type Out;
    fn get_data(&self) -> Self::Out;
}

impl DataRef for () {
    type Out = impl Iterator<Item = &'static str>;
    fn get_data(&self) -> Self::Out {
        DATA.iter().cloned()
    }
}

fn main() {
    let _ = ().get_data().collect::<Vec<_>>();
}