#![allow(unused_macro_rules)]
#![feature(trait_alias)]

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

fn sum(items: Vec<Box<dyn IdentMap>>) -> i32 {
    items.iter().map(|x| x.map("main").unwrap_or(0)).sum()
}

fn main() {
    let map1 = Box::new(MyMap);
    let map2 = Box::new(MyMap);

    let total = sum(vec![map1, map2]);
    println!("Total: {}", total);
}