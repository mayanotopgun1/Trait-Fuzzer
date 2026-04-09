struct WithDrop;

trait DropExt {
    fn custom_drop(&mut self);
}

impl Drop for WithDrop {
    fn drop(&mut self) {}
}

impl DropExt for WithDrop {
    fn custom_drop(&mut self) {}
}

trait ConsumeExt<'a> {
    fn consume(self) -> &'a mut ();
}

impl<'a> ConsumeExt<'a> for (&'a mut (), WithDrop) {
    fn consume(self) -> &'a mut () { self.0 }
}

fn main() {}