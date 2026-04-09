trait MainTrait {
    fn execute(self);
}

impl MainTrait for () {
    fn execute(self) {}
}

fn main() {
    ().execute();
}