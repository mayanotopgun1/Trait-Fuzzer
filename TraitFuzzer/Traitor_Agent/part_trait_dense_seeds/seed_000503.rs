macro_rules! as_stmt { ($s:stmt) => { $s }; }

trait Declare {
    fn declare(&self);
}

impl Declare for () {
    fn declare(&self) {
        let _x = 0u32;
    }
}

fn main() {
    as_stmt!(().declare());
}