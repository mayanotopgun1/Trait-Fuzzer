#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod kitty {
    pub struct cat {
        meows: usize,
        name: String,
    }

    // Make the trait public to allow its use outside of the module
    pub trait NameGetter {
        fn get_name(&self) -> String;
    }

    impl NameGetter for cat {
        fn get_name(&self) -> String { self.name.clone() }
    }

    pub fn cat(in_name: String) -> cat {
        cat {
            name: in_name,
            meows: 0
        }
    }
}

use kitty::{cat, NameGetter}; // Import the trait and the struct to bring them into scope

pub fn main() {
  assert_eq!(kitty::cat("Spreckles".to_string()).get_name(),
                 "Spreckles".to_string());
}