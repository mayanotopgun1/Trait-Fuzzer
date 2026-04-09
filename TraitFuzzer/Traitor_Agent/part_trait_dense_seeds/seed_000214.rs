trait ExtractValue { fn extract(&self) -> i32; }

impl ExtractValue for Result<i32, i32> {
    fn extract(&self) -> i32 {
        match self {
            Ok(v) | Err(v) => *v,
        }
    }
}

fn process_items(items: Vec<Result<i32, i32>>) -> impl Iterator<Item = i32> {
    items.into_iter().map(|item| item.extract())
}

fn main() {
    let v = vec![Ok(2), Err(3), Ok(5)];
    let mut w = Vec::new();
    for item in &v {
        w.push(item.extract());
    }
    let u: Vec<i32> = process_items(v).collect();
    assert_eq!(w, [2, 3, 5]);
    assert_eq!(u, [2, 3, 5]);
}