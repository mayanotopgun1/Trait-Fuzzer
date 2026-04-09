trait Extractor {
    type Item;
    fn extract(&self) -> Self::Item;
}

impl Extractor for (u32, u32) {
    type Item = u32;
    fn extract(&self) -> u32 {
        let ((0, a) | (a, _)) = *self;
        a
    }
}

pub fn f(x: (u32, u32)) {
    let _ = || {
        x.extract()
    };
}

fn main() {}