type A = [u32; const { 2 }];

trait ArrayAccess {
    fn first(&self) -> u32;
}

impl ArrayAccess for A {
    fn first(&self) -> u32 {
        self[0]
    }
}

fn main() {}