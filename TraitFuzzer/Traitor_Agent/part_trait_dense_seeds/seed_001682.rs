struct S;

trait Method {
    fn method(&self) -> bool;
}

impl Method for S {
    fn method(&self) -> bool {
        unimplemented!()
    }
}

fn get<T: Method>() -> T {
    unimplemented!()
}

fn main() {
    match get::<S>() {
        x if x.method() => {}
        _ => {}
    }
}