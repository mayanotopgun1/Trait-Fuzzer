trait TupleAccess<T> {
    fn first(&self) -> T;
    fn last(&self) -> T;
    fn second_last(&self) -> T;
}

impl TupleAccess<u8> for (u8, u8, u8, u8, u8) {
    fn first(&self) -> u8 { self.0 }
    fn last(&self) -> u8 { self.4 }
    fn second_last(&self) -> u8 { self.3 }
}

impl TupleAccess<u8> for (u8, u8, u8, u8) {
    fn first(&self) -> u8 { self.0 }
    fn last(&self) -> u8 { self.3 }
    fn second_last(&self) -> u8 { self.2 }
}

impl TupleAccess<u8> for (u8, u8, u8) {
    fn first(&self) -> u8 { self.0 }
    fn last(&self) -> u8 { self.2 }
    fn second_last(&self) -> u8 { self.1 }
}

impl TupleAccess<u8> for (u8, u8) {
    fn first(&self) -> u8 { self.0 }
    fn last(&self) -> u8 { self.1 }
    fn second_last(&self) -> u8 { self.0 }
}

struct S(u8, u8, u8, u8, u8);

impl TupleAccess<u8> for S {
    fn first(&self) -> u8 { self.0 }
    fn last(&self) -> u8 { self.4 }
    fn second_last(&self) -> u8 { self.3 }
}

trait TupleExt<T>: TupleAccess<T> where T: Copy {
    fn as_vec(&self) -> Vec<T> {
        vec![self.first(), self.second_last(), self.last()]
    }
}

impl<U, T> TupleExt<T> for U where U: TupleAccess<T>, T: Copy {}

fn tuple() {
    let x = (1, 2, 3, 4, 5);
    match &x {
        y => {
            assert_eq!(y.first(), 1);
            assert_eq!(y.second_last(), 4);
            assert_eq!(y.last(), 5);
        }
    }
    match &x {
        y => {
            assert_eq!(y.first(), 1);
            assert_eq!(y.1, 2);
            assert_eq!(y.2, 3);
            assert_eq!(y.last(), 5);
        }
    }
}

fn tuple_struct() {
    let x = S(1, 2, 3, 4, 5);
    match &x {
        y => {
            assert_eq!(y.first(), 1);
            assert_eq!(y.second_last(), 4);
            assert_eq!(y.last(), 5);
        }
    }
    match &x {
        y => {
            assert_eq!(y.first(), 1);
            assert_eq!(y.1, 2);
            assert_eq!(y.2, 3);
            assert_eq!(y.last(), 5);
        }
    }
}

fn main() {
    tuple();
    tuple_struct();
}