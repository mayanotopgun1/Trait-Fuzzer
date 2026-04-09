#![feature(generic_associated_types, specialization)]

trait GetRef<'a> {
    type Out;
    fn get(&self) -> &'a isize;
}

trait AddWithExt<'a, G: GetRef<'a>> {
    fn add_with_ext(&self, other: &G) -> isize;
}

impl<'a, T: GetRef<'a>, G: GetRef<'a>> AddWithExt<'a, G> for T
where
    <T as GetRef<'a>>::Out: std::ops::Add<<G as GetRef<'a>>::Out, Output = isize>,
{
    fn add_with_ext(&self, other: &G) -> isize {
        *self.get() + *other.get()
    }
}

struct Box<'a> {
    value: &'a isize,
}

impl<'a> GetRef<'a> for Box<'a> {
    type Out = isize;

    fn get(&self) -> &'a isize {
        self.value
    }
}

fn main() {
    let a = Box { value: &10 };
    let b = Box { value: &20 };
    println!("Result: {}", a.add_with_ext(&b)); // Output: Result: 30
}