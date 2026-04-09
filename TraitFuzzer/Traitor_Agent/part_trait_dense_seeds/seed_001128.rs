#![feature(return_position_impl_trait_in_trait)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod kitty {
    pub struct cat {
        meows: usize,
        name: String,
    }

    pub trait NameGetter {
        fn get_name(&self) -> impl std::fmt::Display;
    }

    impl NameGetter for cat {
        fn get_name(&self) -> impl std::fmt::Display { self.name.clone() }
    }

    pub fn cat(in_name: String) -> cat {
        cat {
            name: in_name,
            meows: 0
        }
    }
}

use kitty::{cat, NameGetter};

pub fn main() {
    assert_eq!(kitty::cat("Spreckles".to_string()).get_name().to_string(), "Spreckles");
}