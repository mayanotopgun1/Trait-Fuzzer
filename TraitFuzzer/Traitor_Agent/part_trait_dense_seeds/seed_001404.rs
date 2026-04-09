trait AddExt {
    fn add_one(&self) -> Self;
}

impl AddExt for i32 {
    fn add_one(&self) -> Self {
        self + 1
    }
}

fn test() {
    let _x = 1.add_one();
}

trait ReturnExt {
    fn return_value(&self) -> u32;
}

impl ReturnExt for () {
    fn return_value(&self) -> u32 {
        1.add_one() as u32
    }
}

fn test2() -> u32 {
    let _unit = (); // Use a variable name instead of `_`
    _unit.return_value()
}

fn main() {}