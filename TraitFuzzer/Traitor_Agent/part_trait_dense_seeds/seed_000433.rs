#![deny(exported_private_dependencies)]

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

fn main() {
    let pt = PublicType { field: Some(42) };
    println!("{:?}", pt.get_field());
}