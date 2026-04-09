#![allow(incomplete_features)]
#![feature(const_closures, const_trait_impl, type_alias_impl_trait)]

trait ArrayCreator<const N: usize> {
    type Out;
    fn create_array(f: impl FnMut(usize) -> u32 + Copy) -> Self::Out;
}

type ArrayType<const N: usize> = [u32; N];

impl<const N: usize> ArrayCreator<N> for () {
    type Out = ArrayType<N>;
    fn create_array(mut f: impl FnMut(usize) -> u32 + Copy) -> Self::Out {
        let mut array = [0; N];
        let mut i = 0;
        loop {
            array[i] = f(i);
            i += 1;
            if i == N {
                break;
            }
        }
        array
    }
}

fn main() {
    let x = <() as ArrayCreator<5>>::create_array(|i| 2 * i as u32);
    assert_eq!(x, [0, 2, 4, 6, 8]);

    let y = <() as ArrayCreator<5>>::create_array(|i| 2 * i as u32 + 1);
    assert_eq!(y, [1, 3, 5, 7, 9]);
}