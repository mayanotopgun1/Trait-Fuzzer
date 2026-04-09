#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod to_reuse {
    pub trait Types<T, U> {
        type Out;
        fn types(x: U, y: T) -> Self::Out;
    }

    impl<T, U> Types<T, U> for () {
        type Out = (T, U);
        fn types(x: U, y: T) -> Self::Out {
            (y, x)
        }
    }

    pub trait Late<'a, 'b>: Types<&'a u8, &'b u8> {
        type OutLate<'c> where Self: 'c;
        fn late(x: &'a u8, y: &'b u8) -> Self::OutLate<'a>;
    }

    impl<'a, 'b> Late<'a, 'b> for () {
        type OutLate<'c> = &'c u8 where Self: 'c;
        fn late(x: &'a u8, y: &'b u8) -> Self::OutLate<'a> {
            x
        }
    }

    pub trait Early<'a>: Types<&'a str, ()> {
        type OutEarly<'c> where Self: 'c;
        fn early(x: &'a str) -> Self::OutEarly<'a>;
    }

    impl<'a> Early<'a> for () {
        type OutEarly<'c> = &'c str where Self: 'c;
        fn early(x: &'a str) -> Self::OutEarly<'a> {
            x
        }
    }
}

fn main() {
    assert_eq!(<() as to_reuse::Types<&'static str, i32>>::types(0, "str"), ("str", 0));
    assert_eq!(<() as to_reuse::Late>::late(&1u8, &2u8), &1u8);
    {
        let s: &'static str = "hello world";
        assert_eq!(<() as to_reuse::Early>::early(s), "hello world");
    }
}