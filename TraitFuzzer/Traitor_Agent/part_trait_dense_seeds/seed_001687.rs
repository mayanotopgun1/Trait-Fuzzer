macro_rules! make_struct {
    ($name:ident) => {
        trait Bar {
            fn field(&self, value: bool);
        }

        #[derive(Debug)]
        struct Foo {
            #[cfg(not(FALSE))]
            field_fn: fn(bool),
        }

        impl Bar for Foo {
            fn field(&self, value: bool) {
                (self.field_fn)(value)
            }
        }
    }
}

make_struct!(param_name);

fn main() {}