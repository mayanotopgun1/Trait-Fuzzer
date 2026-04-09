#![feature(extern_item_impls)]

pub mod a {
    #[eii(foo)]
    pub fn foo();

    pub trait FooTrait { fn foo(&self); }
    impl FooTrait for () { fn foo(&self) { foo(); } }
}

pub mod b {
    #[eii(foo)]
    pub fn foo();

    pub trait FooTrait { fn foo(&self); }
    impl FooTrait for () { fn foo(&self) { foo(); } }
}

#[a::foo]
fn a_foo_impl() {
    println!("foo1");
}

#[b::foo]
fn b_foo_impl() {
    println!("foo2");
}

fn main() {
    let _: &dyn a::FooTrait = &();
    let _: &dyn b::FooTrait = &();

    a::foo();
    b::foo();
}