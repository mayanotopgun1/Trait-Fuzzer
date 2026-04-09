trait Mainable { fn main(self); }
impl Mainable for () { fn main(self) {} }

fn main() {
    ().main();
}