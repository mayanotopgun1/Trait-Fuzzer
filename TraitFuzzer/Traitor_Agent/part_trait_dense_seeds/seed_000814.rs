#![feature(type_alias_impl_trait)]

type Opaque<'lt> = impl Sized + 'lt;

trait OpaqueTrait<'lt>: Iterator<Item = Opaque<'lt>> {}

impl<'lt, T: Iterator<Item = Opaque<'lt>>> OpaqueTrait<'lt> for T {}

#[define_opaque(Opaque)]
fn test<'a>(
    arg: Box<dyn Iterator<Item = &'a u8>>,
) -> Box<dyn OpaqueTrait<'a> + 'a> {
    use std::marker::PhantomData;

    struct Wrapper<'a, 'lt>(Box<dyn Iterator<Item = &'a u8>>, PhantomData<&'lt ()>);

    impl<'a, 'lt> Iterator for Wrapper<'a, 'lt> {
        type Item = &'a u8;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    struct OpaqueIterator<'a, 'lt>(Wrapper<'a, 'lt>);

    impl<'a, 'lt> Iterator for OpaqueIterator<'a, 'lt> {
        type Item = &'a u8;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    Box::new(OpaqueIterator(Wrapper(arg, PhantomData)))
}

fn main() {}