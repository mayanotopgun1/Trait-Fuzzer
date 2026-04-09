trait ConstValue {
    const VALUE: u8;
}

#[cfg(false)]
struct XStruct;

#[cfg(false)]
impl ConstValue for XStruct {
    const VALUE: u8 = 0;
}

fn main() {}