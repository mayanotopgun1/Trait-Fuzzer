trait Displayable {
    fn display(&self) -> String;
}

impl Displayable for String {
    fn display(&self) -> String {
        self.clone()
    }
}

fn main() {
    let str_var: String = "meh".to_string();
    panic!("{}", str_var.display());
}