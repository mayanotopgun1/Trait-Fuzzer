#![feature(impl_trait_in_assoc_type)]

const USES_PTR: () = { let u = (); &raw const u; };
static ALSO_USES_PTR: () = { let u = (); &raw const u; };

trait PtrAccess {
    fn raw_ptr(&self) -> *const Self;
}

impl PtrAccess for i32 {
    fn raw_ptr(&self) -> *const Self {
        self as *const _
    }
}

fn create_array() -> impl Iterator<Item = i32> {
    (0..4).map(|x| x + 1)
}

fn main() {
    let x: Vec<i32> = create_array().collect();
    let mut one = 1;
    let two = 2;
    if &raw const one == &raw mut one {
        match &raw const two {
            _ => {}
        }
    }
    let three = 3;
    let mut four = 4;
    println!("{:p}", &raw const three);
    unsafe { &raw mut four; }
}