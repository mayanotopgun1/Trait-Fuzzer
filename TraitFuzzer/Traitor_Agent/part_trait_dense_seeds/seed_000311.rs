#![feature(asm_sym, generic_associated_types)]

trait GlobalAsm {
    type Sym;
    fn global_asm(_: &'static str, sym: Self::Sym, _: ());
}

impl GlobalAsm for () {
    type Sym = &'static ();
    fn global_asm(_: &'static str, sym: Self::Sym, _: ()) {}
}

pub fn main() {}