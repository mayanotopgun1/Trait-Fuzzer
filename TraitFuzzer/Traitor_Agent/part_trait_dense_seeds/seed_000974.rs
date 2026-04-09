#![allow(dead_code)]

trait CheckResult<T> {
    fn check(self) -> bool;
}

impl<T, E> CheckResult<T> for Result<T, E> {
    fn check(self) -> bool {
        self.is_err()
    }
}

impl<T> CheckResult<Option<T>> for Option<T> {
    fn check(self) -> bool {
        self.is_none()
    }
}

fn none() -> bool {
    let opt = Some("test");
    if opt.check() {
        return true;
    }
    false
}

fn ok() -> bool {
    let res = Err::<(), &'static str>("test");
    if res.check() {
        return true;
    }
    false
}

pub fn main() {
    let x = Some(3);
    let Some(y) = x else {
        panic!("let-else panicked");
    };
    assert_eq!(y, 3);
    let Some(_) = x else {
        panic!("bad match");
    };
    assert!(none());
    assert!(ok());

    assert!((|| {
        let value = 2_i32;
        if value != 1 {
            return true;
        }
        false
    })());

    enum Foo {
        One,
        Two(usize),
        Three(String, isize),
    }

    trait CheckFoo {
        fn check_one(&self) -> bool;
        fn check_two(&self) -> bool;
        fn check_three(&self) -> bool;
    }

    impl CheckFoo for Foo {
        fn check_one(&self) -> bool {
            matches!(self, Foo::One)
        }

        fn check_two(&self) -> bool {
            matches!(self, Foo::Two(_))
        }

        fn check_three(&self) -> bool {
            matches!(self, Foo::Three(_, _))
        }
    }

    let foo = Foo::Three("three".to_string(), 42);
    let one = || {
        if !foo.check_one() {
            println!("Not One");
        }
    };
    let two = || {
        if !foo.check_two() {
            println!("Not Two");
        }
    };
    let three = || {
        match foo {
            Foo::Three(_, _) => println!("Is Three"),
            _ => println!("Not Three"),
        }
    };

    one();
    two();
    three();

    let a@Foo::Two(_) = Foo::Two(42_usize) else {
        panic!("bad match")
    };
    let Foo::Two(b) = a else {
        panic!("panic in nested `if let`")
    };
    assert_eq!(b, 42_usize);
}