#![warn(rustc::internal)]

trait HashMapExt<K, V> {
    fn new_extended() -> Box<Self>;
}

impl<K, V> HashMapExt<K, V> for std::collections::HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new_extended() -> Box<Self> {
        Box::new(Self::new())
    }
}

#[allow(rustc::foo::default_hash_types)]
fn main() {
    let _ = <std::collections::HashMap<String, String>>::new_extended();
}