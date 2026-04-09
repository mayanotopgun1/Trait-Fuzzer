#![crate_type = "proc-macro"]

#[cfg(target_feature = "crt-static")]
compile_error!("crt-static is enabled");

extern crate proc_macro;

use proc_macro::TokenStream;

trait DeriveFoo {
    fn derive_foo(self, input: TokenStream) -> TokenStream;
}

impl DeriveFoo for () {
    fn derive_foo(self, input: TokenStream) -> TokenStream {
        input
    }
}

#[proc_macro_derive(Foo)]
pub fn derive_foo(input: TokenStream) -> TokenStream {
    ().derive_foo(input)
}