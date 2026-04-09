#![feature(impl_trait_in_assoc_type)]

trait TakeEdgeCounters {
    fn take_edge_counters(&mut self) -> Option<impl Iterator<Item = i32>>;
}

impl TakeEdgeCounters for Option<Vec<i32>> {
    fn take_edge_counters(&mut self) -> Option<impl Iterator<Item = i32>> {
        self.take().map(|m| m.into_iter())
    }
}

fn main() {}