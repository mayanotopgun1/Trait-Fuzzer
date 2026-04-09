#![feature(generic_associated_types)]

trait IterExt: Sized + Iterator {
    type ItemArray<const N: usize>;
    
    fn default_for_size<const N: usize>(self) -> Self::ItemArray<N>
    where
        Self::ItemArray<N>: Default,
    {
        Default::default()
    }
}

impl<T: Iterator> IterExt for T {
    type ItemArray<const N: usize> = [T::Item; N];
}

fn main(){
    const N: usize = 10;
    let arr = (0u32..10).default_for_size::<N>();
    assert_eq!(arr, [0; 10]);
}