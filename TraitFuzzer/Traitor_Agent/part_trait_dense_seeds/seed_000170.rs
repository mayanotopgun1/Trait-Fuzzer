#![allow(non_upper_case_globals)]

enum TestOption<T> {
    TestNone,
    TestSome(T),
}

pub struct Request {
    foo: TestOption<u64>,
    bar: u8,
}

trait RequestTrait {
    fn get_foo(&self) -> &TestOption<u64>;
    fn get_bar(&self) -> u8;
}

impl RequestTrait for Request {
    fn get_foo(&self) -> &TestOption<u64> {
        &self.foo
    }
    fn get_bar(&self) -> u8 {
        self.bar
    }
}

fn default_instance() -> &'static dyn RequestTrait {
    static instance: Request = Request {
        foo: TestOption::TestNone,
        bar: 17,
    };
    &instance as &dyn RequestTrait
}

fn non_default_instance() -> &'static dyn RequestTrait {
    static instance: Request = Request {
        foo: TestOption::TestSome(0x1020304050607080),
        bar: 19,
    };
    &instance as &dyn RequestTrait
}

pub fn main() {
    let default = default_instance();
    match (default.get_foo(), default.get_bar()) {
        (&TestOption::TestNone, 17) => {},
        _ => panic!(),
    };

    let non_default = non_default_instance();
    match (non_default.get_foo(), non_default.get_bar()) {
        (&TestOption::TestSome(0x1020304050607080), 19) => {},
        _ => panic!(),
    };
}