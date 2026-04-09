pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

trait MyNum : Sized + Add<Self, Self> { }

struct MyInt { val: isize }

impl Add<MyInt, MyInt> for MyInt {
    fn add(&self, other: &MyInt) -> MyInt {
        mi(self.val + other.val)
    }
}

impl MyNum for MyInt {}

trait DoubleAdd<T: MyNum>: Add<T, T> {
    fn double_add(&self, rhs: &T) -> T {
        let first_add = self.add(rhs);
        first_add.add(rhs)
    }
}

impl<U: MyNum> DoubleAdd<U> for U where U: Add<U, U> {}

trait AddExt<T: MyNum>: Add<T, T> {
    fn add_ext(&self, rhs: &T) -> T {
        let result = self.add(rhs);
        result
    }
}

impl<U: MyNum> AddExt<U> for U where U: Add<U, U> {}

fn f<T: MyNum + AddExt<T>>(x: T, y: T) -> T {
    return x.add_ext(&y);
}

fn mi(v: isize) -> MyInt {
    MyInt { val: v }
}

pub fn main() {
    let (x, y) = (mi(3), mi(5));
    let z = f(x, y);
    assert_eq!(z.val, 8);
}