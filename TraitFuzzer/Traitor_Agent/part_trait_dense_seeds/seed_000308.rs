trait Tc {}
impl Tc for bool {}

trait TcRef { fn as_ref(&self) -> &dyn Tc; }
impl<T: Tc> TcRef for T { fn as_ref(&self) -> &dyn Tc { self } }

fn main() {
    let _: &[&dyn TcRef] = &[&true];
}