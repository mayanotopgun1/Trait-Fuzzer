#![crate_type = "lib"]

trait ArgCollector {
    fn collect_args(&self) -> Vec<String>;
}

impl ArgCollector for () {
    fn collect_args(&self) -> Vec<String> {
        std::env::args().skip(1).collect()
    }
}

pub fn foo() -> Vec<String> {
    let collector = ();
    collector.collect_args()
}