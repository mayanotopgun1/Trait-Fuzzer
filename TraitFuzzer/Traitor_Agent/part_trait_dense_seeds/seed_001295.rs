trait Print { fn print(&self); }
impl Print for &str { fn print(&self) { println!("{}", self); } }

fn main() {
    let message = "a valid shebang (that is also a rust comment)";
    message.print();
}