#![allow(dead_code, unused)]

trait Test {
    fn foo(&self) { }
}

struct SomeStruct<'a> {
    t: &'a Box<dyn Test>,
    u: &'a Box<dyn Test+'a>,
}

impl<'a> SomeStruct<'a> {
    fn update_t(&mut self, new_t: &'a Box<dyn Test>) {
        self.t = new_t;
    }

    fn update_u(&mut self, new_u: &'a Box<dyn Test+'a>) {
        self.u = new_u;
    }
}

fn a<'a>(t: &'a Box<dyn Test>, mut ss: SomeStruct<'a>) {
    ss.update_t(t);
}

fn b<'a>(t: &'a Box<dyn Test>, mut ss: SomeStruct<'a>) {
    ss.update_u(t);
}

fn d<'a>(t: &'a Box<dyn Test+'a>, mut ss: SomeStruct<'a>) {
    ss.update_u(t);
}

fn main() {
}