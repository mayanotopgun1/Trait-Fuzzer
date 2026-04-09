#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

trait MatchTuple {
    type Output: core::fmt::Debug;
    fn match_first(&self) -> Option<Self::Output>;
}

trait MatchTupleExt: MatchTuple {
    fn has_first(&self) -> bool {
        self.match_first().is_some()
    }
}

impl<T> MatchTupleExt for T where T: MatchTuple {}

impl MatchTuple for &(Option<i32>, (), (), Vec<i32>) {
    type Output = i32;

    fn match_first(&self) -> Option<Self::Output> {
        self.0.clone()
    }
}

pub fn main() {
    let foo = (Some(1), (), (), vec![2, 3]);

    if (&foo).has_first() {
        let n: i32 = (&foo).match_first().unwrap();
        assert_eq!(foo.3.len(), 2);
        assert_eq!(n, 1);
    } else {
        panic!()
    }

    match foo {
        (None, (), (), ..) => panic!(),
        (Some(_), _, _, _) => todo!(),
    }
}