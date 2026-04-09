use std::collections::HashMap;
use std::path::{Path, PathBuf};

trait MapKey<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K, V> MapKey<K, V> for HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
}

fn main() {
    let m: HashMap<PathBuf, ()> = HashMap::new();
    let k = Path::new("foo");
    println!("{:?}", m.get(k));
}