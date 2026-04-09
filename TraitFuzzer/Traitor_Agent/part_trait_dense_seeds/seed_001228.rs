#![feature(return_position_impl_trait_in_trait)]

trait Displayable {
    fn display(&self) -> impl std::fmt::Display;
}

impl Displayable for String {
    fn display(&self) -> impl std::fmt::Display {
        self.clone()
    }
}

fn main() {
    let str_var: String = "meh".to_string();
    panic!("{}", str_var.display());
}