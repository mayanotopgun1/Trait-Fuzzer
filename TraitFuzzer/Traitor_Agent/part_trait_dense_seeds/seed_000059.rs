#![feature(generic_associated_types)]

pub trait Sequence<Item, Subsequence: ?Sized> {
    type Iter<'a>: Iterator<Item = Item>
    where
        Self: 'a;
}

pub trait NodeWalk<Graph: GraphBase, NodeSubwalk: ?Sized>: Sequence<Graph::NodeIndex, NodeSubwalk> {}

pub trait GraphBase {
    type NodeIndex;
}

pub trait WalkableGraph: GraphBase {}

trait NodeCount<Graph: GraphBase>: NodeWalk<Graph, Self> + Sized {
    fn count_nodes(&self) -> usize where Self: Sized;
}

impl<T, G> NodeCount<G> for T
where
    T: NodeWalk<G, Self>,
    G: GraphBase,
{
    fn count_nodes(&self) -> usize {

        0
    }
}

trait NodeCountExt<Graph: GraphBase>: NodeCount<Graph> {
    fn verbose_count_nodes(&self) -> String where Self: Sized {
        format!("Node count: {}", self.count_nodes())
    }
}

impl<T, G> NodeCountExt<G> for T
where
    T: NodeCount<G>,
    G: GraphBase,
{
}

fn main() {}