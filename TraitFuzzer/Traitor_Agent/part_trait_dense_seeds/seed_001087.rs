trait Trait {
    fn provided();
}

struct Type;

impl Type {
    pub fn perform() {}
}

impl Trait for Type {
    fn provided() {}
}

fn main() {
    let _ = Type::perform();
}