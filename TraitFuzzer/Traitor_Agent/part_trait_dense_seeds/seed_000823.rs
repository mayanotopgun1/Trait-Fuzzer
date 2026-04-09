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
    let mut _val = Noise;
    _val.drop();
    panic!("Test");
}

fn main() {
    panic_in_ffi();
}