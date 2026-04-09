trait FContainer {
    fn get_f(&self) -> &Box<[Option<Self>; 1]>
    where
        Self: Sized;
}

impl FContainer for T {
    fn get_f(&self) -> &Box<[Option<Self>; 1]> {
        &self.f
    }
}

struct T {
    f: Box<[Option<T>; 1]>
}

fn main() {
    let x = T { f: Box::new([None]) };
    let _ = x.get_f();
}