use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Scope<'a>(&'a PhantomData<&'a mut &'a ()>);

trait EventDispatch<'a> {
    fn dispatch(&self, scope: Scope<'a>, f: impl FnMut() + 'a);
}

impl<'a> EventDispatch<'a> for Scope<'a> {
    fn dispatch(&self, scope: Scope<'a>, f: impl FnMut() + 'a) {
        event(scope, f);
    }
}

fn make_fn<'a>(_: Scope<'a>) -> impl Fn() + Copy + 'a {
    || {}
}

trait MakeFn<'a> {
    fn make_fn(&self) -> impl Fn() + Copy + 'a;
}

impl<'a> MakeFn<'a> for Scope<'a> {
    fn make_fn(&self) -> impl Fn() + Copy + 'a {
        make_fn(*self)
    }
}

fn foo(cx: Scope) {
    let open_toggle = cx.make_fn();

    || cx.dispatch(cx, open_toggle);
}

fn event<'a, F: FnMut() + 'a>(_: Scope<'a>, _: F) {}

fn main() {}