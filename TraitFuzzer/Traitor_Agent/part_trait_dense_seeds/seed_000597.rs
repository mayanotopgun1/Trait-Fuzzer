trait Identity { fn identity(&self) -> Self; }
impl Identity for u32 { fn identity(&self) -> Self { *self } }

fn r#fn(r#match: u32) -> u32 {
    r#match.identity()
}

pub fn main() {
    let r#struct = 1;
    assert_eq!(1, r#struct);

    let foo = 2;
    assert_eq!(2, foo);

    let r#bar = 3;
    assert_eq!(3, r#bar);

    assert_eq!(4, r#fn(4));

    let r#true = false;
    assert_eq!(r#true, false);
}