trait A<T> {
    fn g<U>(&self, x: T, y: U) -> (T, U);
}

impl<T> A<T> for i32 {
    fn g<U>(&self, x: T, y: U) -> (T, U) { (x, y) }
}

impl<T> A<T> for u32 {
    fn g<U>(&self, x: T, y: U) -> (T, U) { (x, y) }
}

fn f<T, U, V: A<T>>(i: V, j: T, k: U) -> (T, U) {
    i.g(j, k)
}

pub fn main() {
    assert_eq!(f(0_i32, 1, 2), (1, 2));
    assert_eq!(f(0_u32, 1, 2), (1, 2));
}