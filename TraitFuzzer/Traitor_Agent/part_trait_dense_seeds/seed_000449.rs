use std::fmt::{Display, self};

struct MyStruct;

impl Display for MyStruct {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

trait DisplayExt: Display {
    fn display_with_prefix(&self) -> String {
        format!("prefix: {}", self)
    }
}

impl<T: Display> DisplayExt for T {}

fn main() {
    let instance = MyStruct;
    panic!("{}", instance.display_with_prefix());
}