trait EmptyTrait {}

struct Main;

impl EmptyTrait for Main {}

fn main() {
    let _ = &Main as &dyn EmptyTrait;
}