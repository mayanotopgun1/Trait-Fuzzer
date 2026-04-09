#![feature(generic_associated_types)]

trait Empty {}
trait DisplayEmpty: Empty + std::fmt::Debug {
    type Output<'a> where Self: 'a;
    fn show(&self) -> Self::Output<'_>;
}

impl<T: Empty + std::fmt::Debug> DisplayEmpty for T {
    type Output<'a> = String where T: 'a;
    fn show(&self) -> Self::Output<'_> {
        format!("{:?}", self)
    }
}

struct Main;
impl Empty for Main {}
impl std::fmt::Debug for Main {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Main")
    }
}

fn main() {
    let m = Main;
    let _ = m.show();
}