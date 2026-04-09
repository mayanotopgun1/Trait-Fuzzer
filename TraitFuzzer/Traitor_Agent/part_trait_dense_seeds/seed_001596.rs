trait StringLike { fn len(&self) -> usize; }
impl StringLike for String { fn len(&self) -> usize { self.len() } }

trait StringDoubling: StringLike {
    fn double_string(&mut self);
}

impl StringDoubling for String {
    fn double_string(&mut self) {
        *self = format!("{}{}", self, self);
    }
}

pub fn main() {

    let mut a: String = "A".to_string();
    let mut i = 20;
    let mut expected_len = 1;
    while i > 0 {
        println!("{}", a.len());
        assert_eq!(a.len(), expected_len);
        a.double_string();
        i -= 1;
        expected_len *= 2;
    }
}