enum Inner { Member(u32) }

trait CreateInner {
    fn create_member(value: u32) -> Self;
}

impl CreateInner for Inner {
    fn create_member(value: u32) -> Self {
        Inner::Member(value)
    }
}

fn main() {
    let _ = Inner::create_member(0);
}