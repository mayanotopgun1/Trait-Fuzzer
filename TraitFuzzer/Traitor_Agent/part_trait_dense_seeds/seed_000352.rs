#![crate_type="lib"]
#![crate_name="xcrate_issue_61711_b"]

trait AliasTrait {
    fn get_alias(&self) -> &'static str;
}

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
}

pub use crate as alias;

fn main() {
    let s = Struct::new();
    println!("{}", s.get_alias());
}