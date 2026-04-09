#![deny(exported_private_dependencies)]
#![feature(impl_trait_in_assoc_type)]

pub struct PublicType {
    pub field: Option<u8>
}

trait FieldAccessor {
    fn get_field(&self) -> &Option<u8>;
}

impl FieldAccessor for PublicType {
    fn get_field(&self) -> &Option<u8> {
        &self.field
    }
}

fn create_accessor(pt: PublicType) -> impl FieldAccessor {
    pt
}

fn main() {
    let pt = PublicType { field: Some(42) };
    let accessor = create_accessor(pt);
    println!("{:?}", accessor.get_field());
}