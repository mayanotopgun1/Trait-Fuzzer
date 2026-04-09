#![allow(dead_code)]

static DATA: &'static [&'static str] = &["my string"];

trait DataRef {
    fn get_data(&self) -> &'static [&'static str];
}

impl DataRef for () {
    fn get_data(&self) -> &'static [&'static str] {
        DATA
    }
}

fn main() {
    let _ = ().get_data();
}