trait Promote<const N: i32> {
    fn promote(&self);
}

impl<const N: i32> Promote<N> for () {
    fn promote(&self) {
        let _ = &N;
    }
}

fn main() {
    <() as Promote<0>>::promote(&());
}