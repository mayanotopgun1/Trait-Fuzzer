trait CountMembers {
    fn count(&self) -> usize;
}

impl CountMembers for [usize] {
    fn count(&self) -> usize {
        match *self {
            []         => 0,
            [_]        => 1,
            [_, ref xs @ ..] => 1 + xs.count()
        }
    }
}

fn main() {
    assert_eq!([1, 2, 3, 4].count(), 4);
}