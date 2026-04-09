pub trait Transmutor {
    type Input;
    type Output;
    unsafe fn transmute(input: Self::Input) -> Self::Output;
}

impl Transmutor for () {
    type Input = [isize; 1];
    type Output = isize;

    unsafe fn transmute(input: Self::Input) -> Self::Output {
        ::std::mem::transmute::<[isize; 1], isize>(input)
    }
}

pub fn main() {
    unsafe {
        let _ = <() as Transmutor>::transmute([1]);
    }
}