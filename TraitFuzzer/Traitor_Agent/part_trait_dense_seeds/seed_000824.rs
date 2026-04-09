struct Noise;

trait NoisyDrop {
    fn drop(&mut self);
}

impl NoisyDrop for Noise {
    fn drop(&mut self) {
        eprintln!("Noisy Drop");
    }
}

extern "C" fn panic_in_ffi() {
    let mut _val: Box<dyn NoisyDrop> = Box::new(Noise);
    // Use the `drop` function to explicitly drop the value
    drop(_val);
    panic!("Test");
}

fn main() {
    panic_in_ffi();
}