struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

trait RefsCollector<T> {
    fn to_refs(&mut self) -> Vec<&mut T>;
}

impl<T> RefsCollector<T> for List<T> {
    fn to_refs(&mut self) -> Vec<&mut T> {
        let mut result = vec![];
        let mut current = self;
        loop {
            result.push(&mut current.value);
            if let Some(n) = current.next.as_mut() {
                current = n;
            } else {
                return result;
            }
        }
    }
}

fn main() {
    let mut list = List { value: 1, next: None };
    let vec = list.to_refs();
    assert_eq!(vec![&mut 1], vec);
}