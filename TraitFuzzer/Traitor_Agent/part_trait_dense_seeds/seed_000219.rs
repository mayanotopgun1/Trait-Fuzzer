trait Iterable { fn iter(&self) -> std::slice::Iter<'_, Self::Item>; type Item; }
impl<T> Iterable for [T] { fn iter(&self) -> std::slice::Iter<'_, T> { self.iter() } type Item = T; }

fn main() {
    let arr1: [i32; 2] = [1, 2];
    let arr2: [f64; 2] = [1.0, 2.0];

    for _ in arr1.iter() {}
    for _ in arr2.iter() {}

    let x: Vec<i32> = vec![1, 2];
    for _ in x.iter() {}
}