trait VecDequeExt {
    fn with_capacity_unbounded() -> Self;
}

impl<T> VecDequeExt for std::collections::VecDeque<T> {
    fn with_capacity_unbounded() -> Self {
        Self::with_capacity(!0)
    }
}

fn main() {
    let _ = std::collections::VecDeque::<String>::with_capacity_unbounded();
}