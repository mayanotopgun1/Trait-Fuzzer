#[derive(Copy, Clone)]
struct Copyable;

enum NonCopy {
    Thing(Copyable),
    #[allow(unused)]
    Other,
}

trait ExtractField {
    fn extract(&self) -> Option<Copyable>;
}

impl ExtractField for NonCopy {
    fn extract(&self) -> Option<Copyable> {
        match self {
            NonCopy::Thing(copyable) => Some(*copyable),
            NonCopy::Other => None,
        }
    }
}

struct Wrapper {
    field: NonCopy,
}

fn let_else() {
    let vec = vec![Wrapper { field: NonCopy::Thing(Copyable) }];
    for item in &vec {
        if let Some(_copyable) = item.field.extract() {}
    }
}

fn if_let() {
    let vec = vec![Wrapper { field: NonCopy::Thing(Copyable) }];
    for item in &vec {
        if let Some(copyable) = item.field.extract() {
            let _copyable = copyable;
        }
    }
}

fn main() {
    let_else();
    if_let();
}