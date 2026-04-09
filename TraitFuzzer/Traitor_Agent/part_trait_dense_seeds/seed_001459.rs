trait TakeEdgeCounters {
    fn take_edge_counters(&mut self) -> Option<impl Iterator<Item = i32>>;
}

impl TakeEdgeCounters for Option<Vec<i32>> {
    fn take_edge_counters(&mut self) -> Option<impl Iterator<Item = i32>> {
        self.take().map_or(None, |m| Some(m.into_iter()))
    }
}

fn main() {}