#![allow(deprecated)]

trait Sleep {
    fn sleep(duration: u32);
}

impl Sleep for () {
    fn sleep(duration: u32) {
        std::thread::sleep_ms(duration);
    }
}

fn main() {
    <() as Sleep>::sleep(250);
}