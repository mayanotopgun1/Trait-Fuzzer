#![feature(generic_associated_types)]

struct Fat<T: ?Sized> {
    f1: isize,
    f2: &'static str,
    ptr: T
}

trait PtrAccess<'a> {
    type Out;
    fn get_ptr(&'a self) -> Self::Out;
}

impl<'a, T: ?Sized + 'a> PtrAccess<'a> for Fat<T> {
    type Out = &'a Self;
    fn get_ptr(&'a self) -> Self::Out {
        self
    }
}

fn main() {
    let fat = Fat { f1: 42, f2: "hello", ptr: [1, 2, 3] };
    println!("f1: {}, f2: {}", fat.f1, fat.f2);
    println!("ptr: {:?}", fat.ptr);

    let mut mutable_fat = Fat { f1: 42, f2: "hello", ptr: [1, 2, 3] };
    mutable_fat.f1 = 99;
    mutable_fat.f2 = "world";
    println!("Modified f1: {}, Modified f2: {}", mutable_fat.f1, mutable_fat.f2);
    mutable_fat.ptr[0] = 4;
    println!("Modified ptr: {:?}", mutable_fat.ptr);

    let boxed_fat: Box<Fat<[isize; 3]>> = Box::new(Fat { f1: 5, f2: "some str", ptr: [1, 2, 3] });
    println!("Boxed fat - f1: {}, f2: {}", boxed_fat.f1, boxed_fat.f2);
    println!("Boxed fat - ptr: {:?}", boxed_fat.ptr);

    let fat_ref = &fat;
    let fat_ptr = fat_ref.get_ptr();
    println!("Fat ptr - f1: {}, f2: {}", fat_ptr.f1, fat_ptr.f2);
}