trait AltLit {
    fn altlit(&self, f: isize) -> isize;
}

impl AltLit for () {
    fn altlit(&self, f: isize) -> isize {
        match f {
            10 => { println!("case 10"); return 20; }
            11 => { println!("case 11"); return 22; }
            _ => panic!("the impossible happened"),
        }
    }
}

pub fn main() {
    let _: () = ();
    assert_eq!(().altlit(10), 20);
    assert_eq!(().altlit(11), 22);
}