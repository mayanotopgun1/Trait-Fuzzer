trait Empty {}
trait DisplayEmpty: Empty + std::fmt::Debug { fn show(&self) -> String { format!("{:?}", self) } }
impl<T: Empty + std::fmt::Debug> DisplayEmpty for T {}

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