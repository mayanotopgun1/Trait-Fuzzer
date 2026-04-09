#![allow(deprecated)]

trait Sleep {
    fn sleep(&self, duration: u32);
}

impl Sleep for () {
    fn sleep(&self, duration: u32) {
        std::thread::sleep_ms(duration);
    }
}

fn main() {
    let _ = <dyn Sleep>::sleep(&(), 250);
}