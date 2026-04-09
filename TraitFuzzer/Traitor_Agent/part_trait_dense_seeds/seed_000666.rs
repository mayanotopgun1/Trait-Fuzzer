#![feature(arbitrary_self_types, trait_alias)]

trait ReceiverExt<T> {
    fn bar(self) -> *const T;
}

trait ReceiverAlias<T>: std::ops::Receiver<Target = T> {}

impl<T, U> ReceiverExt<U> for MyNonNull<T>
where
    Self: ReceiverAlias<T>,
{
    fn bar(self) -> *const U {
        self.0 as *const U
    }
}

struct MyNonNull<T>(*const T);

impl<T> std::ops::Receiver for MyNonNull<T> {
    type Target = T;
}

impl<T> ReceiverAlias<T> for MyNonNull<T> {}

#[allow(dead_code)]
impl<T> MyNonNull<T> {
    fn foo<U>(&self) -> *const U {
        let mnn: MyNonNull<U> = self.cast();
        mnn.bar()
    }
    fn cast<U>(&self) -> MyNonNull<U> {
        MyNonNull(self.0 as *const U)
    }
}

#[repr(transparent)]
struct Foo(usize);
#[repr(transparent)]
struct Bar(usize);

fn main() {
    let a = Foo(3);
    let ptr = MyNonNull(&a);
    let _bar_ptr: *const Bar = ptr.foo();
}