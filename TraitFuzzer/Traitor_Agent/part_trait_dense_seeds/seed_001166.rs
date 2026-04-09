trait MacroLike {
    fn expand(&self, arg: &str);
}

struct MyMacro;

impl MacroLike for MyMacro {
    fn expand(&self, arg: &str) {
        println!("Hello, {}!", arg);
    }
}

fn main() {
    let my_macro = MyMacro;
    my_macro.expand("world");
}