#![crate_type = "lib"]
#![feature(impl_trait_in_assoc_type)]

trait ArgCollector {
    fn collect_args(&self) -> Vec<String>;
}

impl ArgCollector for () {
    fn collect_args(&self) -> Vec<String> {
        std::env::args().skip(1).collect()
    }
}

pub fn foo() -> impl ArgCollector {
    ()
}