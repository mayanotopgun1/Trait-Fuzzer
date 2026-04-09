#![allow(unused_macro_rules)]

trait IdentMap {
    fn map(&self, key: &str) -> Option<i32>;
}

struct MyMap;

impl IdentMap for MyMap {
    fn map(&self, key: &str) -> Option<i32> {
        match key {
            "main" => Some(0),
            _ => None,
        }
    }
}

macro_rules! ident_map {
    ( $name:ident = { $($key:ident => $e:expr,)* } ) => {
        macro_rules! $name {
            $(
                ( $key ) => { $e };
            )*

            () => {};
        }
    };
}

ident_map!(my_map = {
    main => 0,
});

fn main() {
    let map = MyMap;
    match map.map("main") {
        Some(value) => value,
        None => panic!("Key not found"),
    };
}