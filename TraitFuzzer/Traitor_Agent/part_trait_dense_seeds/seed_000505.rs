trait Get {
    fn get(&mut self) -> u32;
}

impl Get for () {
    fn get(&mut self) -> u32 {
        0
    }
}

trait GetAdd: Get {
    fn add_one(&mut self) -> u32 {
        self.get() + 1
    }
}

impl<T> GetAdd for T where T: Get {}

impl<'a, T> Get for &'a mut T
where
    T: Get,
{
    fn get(&mut self) -> u32 {
        T::get(*self)
    }
}

fn foo(n: usize, m: &mut ()) -> impl GetAdd + '_ {
    if n > 0 {
        let mut iter = foo(n - 1, m);
        assert_eq!(iter.get(), 1);
    }
    m
}

fn main() {
    let g = foo(1, &mut ()).get();
    assert_eq!(g, 1);
}