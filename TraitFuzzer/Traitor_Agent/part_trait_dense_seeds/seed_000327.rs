#![feature(generic_associated_types)]
#![feature(extern_item_impls)]
#![feature(decl_macro)]
#![feature(rustc_attrs)]
#![feature(eii_internals)]

#[eii_declaration(bar)]
#[rustc_builtin_macro(eii_shared_macro)]
macro foo() {}

unsafe extern "Rust" {
    safe fn bar<'a>(x: &'a u64) -> &'a u64;
}

trait BarExt {
    type Out<'a>;
    unsafe fn bar_ext<'a>(&self, x: &'a u64) -> Self::Out<'a>;
}

impl BarExt for () {
    type Out<'a> = &'a u64;
    unsafe fn bar_ext<'a>(&self, x: &'a u64) -> Self::Out<'a> {
        bar(x)
    }
}

#[foo]
fn other<'a>(_x: &'a u64) -> &'static u64 {
    &0
}

fn main() {
    unsafe {
        let _ = ().bar_ext(&0);
    }
}