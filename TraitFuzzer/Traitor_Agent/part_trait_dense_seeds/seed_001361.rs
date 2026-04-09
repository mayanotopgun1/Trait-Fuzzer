#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum pattern { tabby, tortoiseshell, calico }
enum breed { beagle, rottweiler, pug }
type name = String;
enum ear_kind { lop, upright }
enum animal { cat(pattern), dog(breed), rabbit(name, ear_kind), tiger }

trait Noiser {
    type NoiseOutput;
    fn noise(&self) -> Self::NoiseOutput;
}

impl Noiser for animal {
    type NoiseOutput = Option<String>;

    fn noise(&self) -> Self::NoiseOutput {
        match self {
            animal::cat(..) => Some("meow".to_string()),
            animal::dog(..) => Some("woof".to_string()),
            animal::rabbit(..) => None,
            animal::tiger => Some("roar".to_string()),
        }
    }
}

pub fn main() {
    assert_eq!(animal::cat(pattern::tabby).noise(), Some("meow".to_string()));
    assert_eq!(animal::dog(breed::pug).noise(), Some("woof".to_string()));
    assert_eq!(animal::rabbit("Hilbert".to_string(), ear_kind::upright).noise(), None);
    assert_eq!(animal::tiger.noise(), Some("roar".to_string()));
}