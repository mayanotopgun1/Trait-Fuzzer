#![feature(generic_associated_types)]
#![feature(never_type)]
#![feature(type_ascription)]
#![deny(unreachable_code)]

trait UnitAccess {
    type Output<'a> where Self: 'a;
    fn get_unit(&self) -> Self::Output<'_>;
}

trait UnitAccessExt: UnitAccess {
    fn double_get(&self) -> (Self::Output<'_>, Self::Output<'_>)
    where
        for<'a> Self::Output<'a>: Copy,
    {
        let v = self.get_unit();
        (v, v)
    }
}

impl<T: UnitAccess> UnitAccessExt for T {}

impl UnitAccess for () {
    type Output<'a> = &'a ();
    fn get_unit(&self) -> Self::Output<'_> {
        &()
    }
}

fn main() {
    let x: (!,);
    let _ = type_ascribe!(x.0, _);

    let unit = ();
    let _ = unit.double_get();
}