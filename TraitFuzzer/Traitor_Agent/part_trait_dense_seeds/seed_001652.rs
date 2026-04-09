macro_rules! m {
    (static $name:ident: $t:ty = $e:expr) => {
        let mut $name: $t = $e;
    }
}

trait VectorExt<T> {
    fn add_element(&mut self, element: T);
}

impl<T> VectorExt<T> for Vec<Vec<T>> {
    fn add_element(&mut self, element: T) {
        self.push(vec![element]);
    }
}

fn main() {
    m! {
        static _x: Vec<Vec<u32>> = vec![]
    }

    _x.add_element(42);
}