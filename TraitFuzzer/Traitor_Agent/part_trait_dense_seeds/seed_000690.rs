#![crate_type = "proc-macro"]
#![allow(private_interfaces)]
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn foo<T>(t: T) -> TokenStream {
    TokenStream::new()
}

trait Project {
    type Assoc;
}

impl Project for () {
    type Assoc = TokenStream;
}

trait ProjectExt: Project {
    fn project(&self) -> Self::Assoc;
}

impl ProjectExt for () {
    fn project(&self) -> Self::Assoc {
        TokenStream::new()
    }
}

#[proc_macro]
pub fn uwu(_input: <() as Project>::Assoc) -> <() as Project>::Assoc {
    ().project()
}