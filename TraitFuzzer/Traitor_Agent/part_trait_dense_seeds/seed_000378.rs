#![warn(redundant_imports)]

mod foo {
    use std::fmt;

    pub struct String;

    impl String {
        pub fn new() -> String {
            String{}
        }
    }

    trait DisplayLike { fn display(&self, f: &mut fmt::Formatter) -> fmt::Result; }
    impl DisplayLike for String {
        fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "String")
        }
    }

    impl fmt::Display for String {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.display(f)
        }
    }
}

fn main() {

    {
        use std::string::String;

        let s = String::new();
        println!("{}", s);
    }

    {

        let s = String::new();
        println!("{}", s);
    }

    {
        use foo::*;

        let s = String::new();
        println!("{}", s);
    }

}