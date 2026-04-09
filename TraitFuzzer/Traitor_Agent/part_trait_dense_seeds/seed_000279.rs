pub trait Proj {
    type Assoc<T>;
}

trait Id {
    type This;
}
impl<T> Id for T {
    type This = T;
}

trait WithFastReject: Proj {
    fn with_fast_reject<U>(&self, x: Self::Assoc<u32>)
    where
        U: Proj<Assoc<i32> = u32>,
    {
        let _: Self::Assoc<_> = x;
    }
}

trait NoFastReject: Proj {
    fn no_fast_reject<U>(&self, x: Self::Assoc<u32>)
    where
        <U as Id>::This: Proj<Assoc<i32> = u32>,
    {
        let _: Self::Assoc<_> = x;
    }
}

impl<T> WithFastReject for T where T: Proj {}
impl<T> NoFastReject for T where T: Proj {}

fn main() {}