trait MaxValue {
    const MAX: Self;
}

impl MaxValue for u8 {
    const MAX: u8 = std::u8::MAX;
}

fn main() {
    let _ = <u8 as MaxValue>::MAX;
}