#![feature(return_position_impl_trait_in_trait)]

const FOO: isize = 10;

trait ZeroSizedTrait {
    fn as_zst(&self) -> impl std::fmt::Debug;
}

impl ZeroSizedTrait for isize {
    fn as_zst(&self) -> impl std::fmt::Debug {
        unsafe { std::mem::transmute::<_, &()>(*self) }
    }
}

fn main() {
    match FOO.as_zst() {
        ZST => 9,
    };
}