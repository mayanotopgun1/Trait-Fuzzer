trait SliceExt<T> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

impl<T, const N: usize> SliceExt<T> for [T; N] {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T> SliceExt<T> for &[T] {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.len()
    }
}

fn main() {
    let s: &[bool] = &[true; 0];
    let s0: &[bool; 0] = &[];
    let s1: &[bool; 1] = &[false; 1];
    let s2: &[bool; 2] = &[false; 2];

    assert!(s0.is_empty());
    assert!(!s1.is_empty());
    assert!(!s2.is_empty());

    assert_eq!(s.len(), 0);
    assert_eq!(s0.len(), 0);
    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 2);

    let [] = s0;
    let [_] = s1;
    let [_, _] = s2;

    let [..] = s;
    let [..] = s0;
    let [..] = s1;
    let [..] = s2;

    let [_, ..] = s1;
    let [.., _] = s1;
    let [_, ..] = s2;
    let [.., _] = s2;

    let [_, _, ..] = s2;
    let [_, .., _] = s2;
    let [.., _, _] = s2;
}