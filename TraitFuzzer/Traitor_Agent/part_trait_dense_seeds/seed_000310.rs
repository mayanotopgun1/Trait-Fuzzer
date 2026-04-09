#![feature(asm_sym)]

trait GlobalAsm {
    fn global_asm(_: &'static str, _: &(&'static ()), _: ());
}

impl GlobalAsm for () {
    fn global_asm(_: &'static str, _: &(&'static ()), _: ()) {}
}

pub fn main() {}