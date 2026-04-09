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
    unsafe fn bar_ext<'a>(&self, x: &'a u64) -> &'a u64;
}

impl BarExt for () {
    unsafe fn bar_ext<'a>(&self, x: &'a u64) -> &'a u64 {
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