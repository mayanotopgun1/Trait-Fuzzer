trait MainLike { fn run(&self); }
trait MainExt: MainLike where Self: Sized { fn double_run(&self) { self.run(); self.run(); } }
impl<T: MainLike + Sized> MainExt for T {}
impl MainLike for () { fn run(&self) {} }
fn main() { ().double_run(); }