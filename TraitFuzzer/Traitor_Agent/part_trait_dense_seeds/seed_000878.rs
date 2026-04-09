#![feature(type_alias_impl_trait)]
#![deny(deprecated)]

#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::*;

trait CompileMacro {
    type Output;
    fn compile(input: TokenStream) -> Self::Output;
}

impl CompileMacro for () {
    type Output = TokenStream;
    fn compile(_: TokenStream) -> Self::Output {
        TokenStream::new()
    }
}

struct HiddenType;

trait Debuggable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl Debuggable for HiddenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hidden")
    }
}

type Hidden = HiddenType;

#[proc_macro]
#[deprecated(since = "1.0.0", note = "test")]
pub fn test_compile_without_warning_with_deprecated(input: TokenStream) -> TokenStream {
    <() as CompileMacro>::compile(input)
}