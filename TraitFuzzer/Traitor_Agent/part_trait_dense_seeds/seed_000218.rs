#![feature(impl_trait_in_assoc_type)]

trait ParallelIterator: Sized {
    type Item;
}

trait IntoParallelIterator {
    type Iter: ParallelIterator<Item = Self::Item>;
    type Item;
}

impl<T: ParallelIterator> IntoParallelIterator for T {
    type Iter = T; // Revert to using the concrete type
    type Item = T::Item;
}

trait MultiZipExt {
    fn foo() where Self: Sized, Self: IntoParallelIterator, Self::Iter: ParallelIterator;
}

macro_rules! multizip_impls {
    ($($T:ident),+) => {
        impl<$($T,)+> MultiZipExt for ($( $T, )+)
            where
                $(
                    $T: IntoParallelIterator,
                    $T::Iter: ParallelIterator<Item = $T::Item>,
                )+
                Self: IntoParallelIterator<Item = ($( $T::Item, )+)>,
        {
            fn foo() {}
        }
    }
}

multizip_impls! { A, B, C, D, E, F, G, H, I, J, K, L }

fn main() {}