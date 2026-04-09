unsafe fn add(a: i32, b: i32) -> i32 {
    a + b
}

trait Operation {
    unsafe fn apply(&self, a: i32, b: i32) -> i32;
}

impl Operation for unsafe fn(i32, i32) -> i32 {
    unsafe fn apply(&self, a: i32, b: i32) -> i32 {
        self(a, b)
    }
}

impl Operation for fn(i32, i32) -> i32 {
    unsafe fn apply(&self, a: i32, b: i32) -> i32 {
        self(a, b)
    }
}

fn main() {
    let foo = match "+" {
        "+" => add,
        "-" => |a, b| (a - b) as i32,
        _ => unimplemented!(),
    };
    assert_eq!(unsafe { foo.apply(5, 5) }, 10);

    let foo = match "-" {
        "-" => |a, b| (a - b) as i32,
        "+" => add,
        _ => unimplemented!(),
    };
    assert_eq!(unsafe { foo.apply(5, 5) }, 0);
}