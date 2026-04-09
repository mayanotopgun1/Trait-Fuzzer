mod foo {
    pub trait BarLike {
        fn bar(&self, offset: usize);
    }

    impl BarLike for () {
        fn bar(&self, _offset: usize) {}
    }

    pub fn bar(offset: usize) {
        let _: &dyn BarLike = &();
        let empty: &() = &();
        (empty as &dyn BarLike).bar(offset);
    }
}

fn main() { foo::bar(0); }