trait FooRunner {
    fn run(&self);
}

struct Too;

impl FooRunner for Too {
    #[cfg(foo = "too")]
    fn run(&self) {}
    
    #[cfg(not(foo = "too"))]
    fn run(&self) {}
}

struct Bar;

impl FooRunner for Bar {
    #[cfg(foo = "bar")]
    fn run(&self) {}
    
    #[cfg(not(foo = "bar"))]
    fn run(&self) {}
}

#[cfg(foo)]
fn foo() {}

fn main() {
    #[cfg(foo = "too")]
    let too = Too;
    #[cfg(foo = "too")]
    too.run();

    #[cfg(foo = "bar")]
    let bar = Bar;
    #[cfg(foo = "bar")]
    bar.run();
}