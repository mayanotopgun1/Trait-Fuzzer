#![feature(generic_const_exprs)]

trait ArrayBuilder<const N: usize, F> {
    fn build_array(f: F) -> Self;
}

impl<const N: usize, F: FnMut(usize) -> f32> ArrayBuilder<N, F> for [f32; N] {
    fn build_array(mut f: F) -> Self {
        let mut result = [0.0; N];
        let mut i = 0;
        while i < N {
            result[i] = f(i);
            i += 1;
        }
        result
    }
}

pub struct TestArray<const N: usize>
where
    [(); N / 2]:,
{
    array: [f32; N / 2],
}

impl<const N: usize> TestArray<N>
where
    [(); N / 2]:,
{
    fn from_fn_2<F: FnMut(usize) -> f32>(f: F) -> Self {
        Self { array: ArrayBuilder::<{ N / 2 }, F>::build_array(f) }
    }
}

fn main() {
    TestArray::<4>::from_fn_2(|_i| 0.0);
}