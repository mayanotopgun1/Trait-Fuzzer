enum E { A, }

const C: [u32; 1] = [1];

trait Accessor {
    fn access(&self, index: usize) -> u32;
}

impl Accessor for [u32; 1] {
    fn access(&self, index: usize) -> u32 {
        self[index]
    }
}

fn main() {
    let a = C.access(E::A as usize);
}