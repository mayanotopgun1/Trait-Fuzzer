#![feature(associated_type_defaults)]

macro_rules! overload {
    ($a:expr, $b:expr) => {
        overload::overload2($a, $b)
    };
    ($a:expr, $b:expr, $c:expr) => {
        overload::overload3($a, $b, $c)
    }
}

fn main() {
    let () = overload!(42, true);

    let r: f32 = overload!("Hello world", 13.0);
    assert_eq!(r, 13.0);

    let () = overload!(42, true, 42.5);

    let r: i32 = overload!("Hello world", 13.0, 42);
    assert_eq!(r, 42);
}

mod overload {

    pub trait Overload {
        type R = ();
        fn overload(self) -> Self::R;
    }

    impl Overload for (i32, bool) {
        fn overload(self) -> Self::R {
            let (a, b) = self;
            println!("i32 and bool {:?}", (a, b));
            ()
        }
    }

    impl<'a> Overload for (&'a str, f32) {
        type R = f32;
        fn overload(self) -> Self::R {
            let (a, b) = self;
            println!("&str and f32 {:?}", (a, b));
            b
        }
    }

    impl Overload for (i32, bool, f32) {
        fn overload(self) -> Self::R {
            let (a, b, c) = self;
            println!("i32 and bool and f32 {:?}", (a, b, c));
            ()
        }
    }

    impl<'a> Overload for (&'a str, f32, i32) {
        type R = i32;
        fn overload(self) -> Self::R {
            let (a, b, c) = self;
            println!("&str and f32 and i32: {:?}", (a, b, c));
            c
        }
    }

    pub trait OverloadExt: Overload {}

    impl<T> OverloadExt for T where T: Overload {}

    pub fn overload2<R, A, B>(a: A, b: B) -> R
    where
        (A, B): Overload<R = R>,
    {
        <(A, B)>::overload((a, b))
    }

    pub fn overload3<R, A, B, C>(a: A, b: B, c: C) -> R
    where
        (A, B, C): Overload<R = R>,
    {
        <(A, B, C)>::overload((a, b, c))
    }
}