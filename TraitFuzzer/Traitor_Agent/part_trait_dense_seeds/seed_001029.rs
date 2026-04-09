#![crate_name = "othercrate"]
#![crate_type = "rlib"]

trait ExposedFunc { fn exposed_func(&self); }

impl ExposedFunc for () {
    fn exposed_func(&self) {
        let result = std::panic::catch_unwind(|| {
            println!("hello!");
        });
        assert!(result.is_ok());
    }
}

fn main() {
    let _ = ().exposed_func();
}