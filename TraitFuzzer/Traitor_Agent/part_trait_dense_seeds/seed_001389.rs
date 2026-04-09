#![feature(type_alias_impl_trait)]

trait OpaqueTrait { fn opaque(&self); }
impl<T: Sized> OpaqueTrait for T { fn opaque(&self) {} }

fn opaque<'a>() -> impl OpaqueTrait + 'static {}
fn assert_static<T: 'static>(_: T) {}

fn test_closure() {
    let closure = |_| {
        assert_static(opaque());
    };
    closure(&opaque());
}

pub type Opaque2 = impl OpaqueTrait + 'static;
pub type Opaque<'a> = Opaque2;
#[define_opaque(Opaque)]
fn define<'a>() -> Opaque<'a> {}

fn test_tait(_: &Opaque<'_>) {
    None::<&'static Opaque<'_>>;
}

fn main() {}