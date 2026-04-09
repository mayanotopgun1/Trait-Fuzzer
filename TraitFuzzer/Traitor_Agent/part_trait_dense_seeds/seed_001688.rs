#![feature(type_alias_impl_trait)]

macro_rules! make_struct {
    ($name:ident) => {
        trait Bar {
            type FieldFn;
            fn field(&self, value: bool);
        }

        #[derive(Debug)]
        struct Foo<F>
        where
            F: Fn(bool),
        {
            #[cfg(not(FALSE))]
            field_fn: F,
        }

        impl<F> Bar for Foo<F>
        where
            F: Fn(bool),
        {
            type FieldFn = F;
            fn field(&self, value: bool) {
                (self.field_fn)(value)
            }
        }
    }
}

make_struct!(param_name);

fn main() {}