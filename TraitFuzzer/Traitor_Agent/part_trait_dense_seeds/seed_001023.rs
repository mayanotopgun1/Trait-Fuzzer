#![feature(cfg_eval)]
#![feature(stmt_expr_attributes)]

trait ZeroProvider { fn zero(&self) -> u32; }
impl ZeroProvider for () { fn zero(&self) -> u32 { #[cfg_eval] #[cfg(not(FALSE))] 0 } }

fn f() -> u32 {
    let _: &dyn ZeroProvider = &();
    #[cfg_eval] #[cfg(not(FALSE))] 0
}

fn main() {}