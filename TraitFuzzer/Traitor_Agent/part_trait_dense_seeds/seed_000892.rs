#![allow(dead_code)]

mod outer {
    pub mod inner {
        pub(in crate::outer) struct Foo;

        trait Bar { fn bar() -> Self; }

        impl Bar for Foo {
            fn bar() -> Foo {
                Foo
            }
        }

        pub fn bar() -> Foo {
            <Foo as Bar>::bar()
        }
    }

    pub mod nested {
        pub mod inner {
            pub(in crate::outer::nested) struct NestedFoo;

            trait Bar { fn bar() -> Self; }

            impl Bar for NestedFoo {
                fn bar() -> NestedFoo {
                    NestedFoo
                }
            }

            pub fn bar() -> NestedFoo {
                <NestedFoo as Bar>::bar()
            }
        }
    }
}

fn main() {}