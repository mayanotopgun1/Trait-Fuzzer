trait MainTrait {
    fn main();
}

impl MainTrait for () {
    fn main() {}
}

fn main() {
    <() as MainTrait>::main();
}