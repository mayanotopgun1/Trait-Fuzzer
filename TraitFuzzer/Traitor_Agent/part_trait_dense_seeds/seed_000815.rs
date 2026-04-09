#[allow(dead_code)]
#[derive(Debug)]
struct Foo {
    value: u32,
}

trait Inspect {
    fn inspect(&self);
}

impl Inspect for Foo {
    fn inspect(&self) {
        println!(
            "I am in thread {:?}, address: {:p}",
            std::thread::current().id(),
            self as *const Foo,
        );
    }
}

struct Wrapper<'a, T>(std::marker::PhantomData<&'a ()>, T);
unsafe impl<T> Sync for Wrapper<'_, T> where T: Sync {}
unsafe impl<'a> std::marker::Sync for Foo {}

fn _assert_sync<T: Sync>() {}

fn inspect(foo: &'static Foo) {
    foo.inspect();
}

fn main() {
    let foo: &'static Foo = &Foo { value: 1 };
    inspect(foo);

    let handle = std::thread::spawn(move || inspect(foo));
    handle.join().unwrap();
}