pub fn poison1() -> impl Sized
where
    (): 'static,
{
}
pub fn poison2() -> impl Sized
where
    (): 'static,
{
    (poison2, ()).define();
}
pub fn poison3() -> impl Sized
where
    (): 'static,
{
}

trait Query {}
impl<Out, F: Fn() -> Out> Query for (F, Out) {}

trait DefineByQuery {
    fn define(&self);
}

impl<Q: Query + Clone> DefineByQuery for Q {
    fn define(&self) {
        define_by_query(self.clone());
    }
}

fn define_by_query(_: impl Query) {}

fn main() {}