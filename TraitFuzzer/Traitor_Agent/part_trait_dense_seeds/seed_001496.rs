trait Panda<T> {
    fn chomp(&self, bamboo: &T) -> T;
}

trait Add<RHS, Result>: Panda<RHS> {
    fn add(&self, rhs: &RHS) -> Result;
}

trait MyNum : Sized + Add<Self, Self> { }

struct MyInt { val: isize }

impl Panda<MyInt> for MyInt {
    fn chomp(&self, bamboo: &MyInt) -> MyInt {
        mi(self.val + bamboo.val)
    }
}

impl Add<MyInt, MyInt> for MyInt {
    fn add(&self, other: &MyInt) -> MyInt { self.chomp(other) }
}

impl MyNum for MyInt {}

trait PandaExt<T>: Panda<T> {
    fn chomp_id(&self, bamboo: &T) -> T where Self: Sized {
        self.chomp(bamboo)
    }
}

impl<S, T> PandaExt<T> for S where S: Panda<T> {}

trait AddExt<RHS, Result>: Add<RHS, Result> {
    fn add_id(&self, rhs: &RHS) -> Result where Self: Sized {
        self.add(rhs)
    }
}

impl<S, RHS, Result> AddExt<RHS, Result> for S where S: Add<RHS, Result> {}

fn f<T: MyNum>(x: T, y: T) -> T {
    return x.add_id(&y).chomp_id(&y);
}

fn mi(v: isize) -> MyInt { MyInt { val: v } }

pub fn main() {
    let (x, y) = (mi(3), mi(5));
    let z = f(x, y);
    assert_eq!(z.val, 13);
}