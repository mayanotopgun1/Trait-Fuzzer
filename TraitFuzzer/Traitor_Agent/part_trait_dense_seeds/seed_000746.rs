pub trait Foo<P> {}

pub trait Bar {
    type Output<'a>: 'a where Self: 'a;
}

impl Foo<i32> for i32 { }

impl<A: Bar> Foo<A::Output<'static>> for A { }

impl Bar for i32 {
    type Output<'a> = &'a u32;
}

trait AdditionalBar: Bar {
    fn double_output() -> Self::Output<'static>
    where
        Self::Output<'static>: std::ops::Add<Output = Self::Output<'static>> + Copy,
    {
        let v = Self::make();
        v + v
    }

    fn make() -> Self::Output<'static>;
}

impl<T: Bar + 'static> AdditionalBar for T
where
    <T as Bar>::Output<'static>: Default,
{
    fn make() -> Self::Output<'static> {
        Default::default()
    }
}

trait AdditionalBarExt: AdditionalBar {
    fn quadruple_output() -> Self::Output<'static>
    where
        Self::Output<'static>: std::ops::Add<Output = Self::Output<'static>> + Copy,
    {
        let v = Self::double_output();
        v + v
    }
}

impl<T: AdditionalBar> AdditionalBarExt for T {}

fn main() {}