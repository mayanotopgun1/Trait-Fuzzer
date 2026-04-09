#![allow(dead_code, unused)]

trait Test {
    fn foo(&self) { }
}

trait TestExt: Test {}

impl<T: Test> TestExt for T {}

struct SomeStruct<'a> {
    t: &'a mut dyn Test,
    u: &'a mut (dyn Test+'a),
}

impl<'a> Test for SomeStruct<'a> {}

fn a<'a>(t: &'a mut dyn Test, mut ss: SomeStruct<'a>) -> impl TestExt + 'a {
    ss.t = t;
    ss
}

fn b<'a>(t: &'a mut (dyn Test+'a), mut ss: SomeStruct<'a>) -> impl TestExt + 'a {
    ss.u = t;
    ss
}

fn c<'a>(t: &'a mut (dyn Test+'a), mut ss: SomeStruct<'a>) -> impl TestExt + 'a {
    ss.t = t;
    ss
}

fn d<'a>(t: &'a mut dyn Test, mut ss: SomeStruct<'a>) -> impl TestExt + 'a {
    ss.u = t;
    ss
}

fn main() {
}