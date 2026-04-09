#![crate_type="lib"]
#![crate_name="xcrate_issue_61711_b"]

#![feature(type_alias_impl_trait)]

trait AliasTrait {
    fn get_alias(&self) -> &'static str;
}

type HiddenAlias = impl AliasTrait;

impl AliasTrait for Struct {
    fn get_alias(&self) -> &'static str {
        "alias"
    }
}

pub struct Struct;

impl Struct {
    pub fn new() -> Self {
        Struct
    }

    #[define_opaque(HiddenAlias)]
    pub fn as_hidden_alias(self) -> HiddenAlias {
        self
    }
}

pub use crate as alias;

fn main() {
    let s = Struct::new();
    println!("{}", s.get_alias());
}