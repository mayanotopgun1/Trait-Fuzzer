trait Print { fn print(&self); }
impl Print for &str { fn print(&self) { println!("{}", self); } }

fn get_message() -> impl Print {
    "a valid shebang (that is also a rust comment)"
}

fn main() {
    let message = get_message();
    message.print();
}