struct Test<T: ?Sized>(T);

trait RefAccess {
    type Target: ?Sized;
    fn get_ref(&self) -> &Self::Target;
}

impl<T: ?Sized> RefAccess for Test<T> {
    type Target = T;
    fn get_ref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = Test([1, 2, 3]);
    let x: &Test<[i32]> = &x;

    let _y = x.get_ref();

    let slice = &[1, 2, 3];
    let x = Test(slice);
    let Test(_slice) = x;
}