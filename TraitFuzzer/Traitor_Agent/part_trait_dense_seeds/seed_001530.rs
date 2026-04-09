#[derive(Debug, PartialEq, Eq)]
struct IntWrapper(u32);

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Default)]
struct HasKeywordField {
    r#struct: u32,
}

struct Generic<r#T>(r#T);

trait Trait<T> {
    fn r#trait(&self) -> T;
}
impl Trait<u32> for Generic<u32> {
    fn r#trait(&self) -> u32 {
        self.0
    }
}

trait UnwrapTrait {
    type Item;
    fn unwrap_trait(self) -> Self::Item;
}
impl UnwrapTrait for IntWrapper {
    type Item = u32;
    fn unwrap_trait(self) -> u32 {
        self.0
    }
}

pub fn main() {
    assert_eq!(IntWrapper(1), r#IntWrapper(1));

    match IntWrapper(2) {
        r#IntWrapper(r#struct) => assert_eq!(2, r#struct),
    }

    assert_eq!("HasKeywordField { struct: 3 }", format!("{:?}", HasKeywordField { r#struct: 3 }));

    let generic = Generic(4);
    assert_eq!(4, generic.0);
    assert_eq!(5, generic.r#trait() + 1);
}