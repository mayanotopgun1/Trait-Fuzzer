trait ForgetFuture: 'static {
    fn forget(self) where Self: Sized;
}

trait FutureExt: ForgetFuture + 'static {}
impl<T: ForgetFuture + 'static> FutureExt for T {}

struct Map<A>(#[allow(dead_code)] A);
impl<A: ForgetFuture> ForgetFuture for Map<A> {
    fn forget(self) where Self: Sized {
        Box::new(Map(self)) as Box<dyn FutureExt>;
    }
}

pub struct Promise;
impl ForgetFuture for Promise {
    fn forget(self) where Self: Sized {
        Box::new(Map(self)) as Box<dyn FutureExt>;
    }
}

fn main() {
    Promise.forget();
}