struct Test;

trait TestTrait {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    #[allow(improper_ctypes_definitions)]
    unsafe extern "C" fn test(&self, val: &str);
}

impl TestTrait for Test {
    unsafe extern "C" fn test(&self, val: &str) {

    }
}

fn main() {}