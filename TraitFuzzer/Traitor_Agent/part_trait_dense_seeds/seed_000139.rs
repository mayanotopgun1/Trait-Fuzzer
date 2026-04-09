trait GetRef<T> { fn get(opt: &Option<T>) -> &T; }

impl<T> GetRef<T> for Option<T> {
    fn get(opt: &Option<T>) -> &T {
        match *opt {
            Some(ref v) => v,
            None => panic!("none")
        }
    }
}

pub fn main() {
    let mut x = Some(23);

    {
        let y = Option::get(&x);
        assert_eq!(*y, 23);
    }

    x = Some(24);

    {
        let y = Option::get(&x);
        assert_eq!(*y, 24);
    }
}