#![feature(return_position_impl_trait_in_trait)]

trait Promote<const N: i32> {
    fn promote(&self) -> impl Fn();
}

impl<const N: i32> Promote<N> for () {
    fn promote(&self) -> impl Fn() {
        move || {
            let _ = &N;
        }
    }
}

fn main() {
    <() as Promote<0>>::promote(&())();
}