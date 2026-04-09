#![feature(type_alias_impl_trait)]
#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]

trait ZeroProvider { fn zero(&self) -> u32; }
impl ZeroProvider for () { fn zero(&self) -> u32 { #[cfg_eval] #[cfg(not(FALSE))] 0 } }

type DynZeroProvider = dyn ZeroProvider;

fn f() -> u32 {
    let _: &DynZeroProvider = &();
    #[cfg_eval] #[cfg(not(FALSE))] 0
}

fn main() {}