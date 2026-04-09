#![allow(unused_mut)]

trait RefsTrait {
    fn get_n(&self) -> isize;
    fn get_refs(&self) -> &[isize];
}

impl RefsTrait for Refs {
    fn get_n(&self) -> isize {
        self.n
    }

    fn get_refs(&self) -> &[isize] {
        &self.refs
    }
}

struct Refs { refs: Vec<isize>, n: isize }

pub fn main() {
    let mut e = Refs{refs: vec![], n: 0};
    let _f = || println!("{}", e.get_n());
    let x: &[isize] = e.get_refs();
    assert_eq!(x.len(), 0);
}