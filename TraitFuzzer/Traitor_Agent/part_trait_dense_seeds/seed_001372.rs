#![feature(return_position_impl_trait_in_trait)]

pub trait Sequence<Item, Subsequence: Sequence<Item, Subsequence>> {}

pub trait NodeWalk<Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>:
    Sequence<Graph::NodeIndex, NodeSubwalk>
{
    fn walk(&self) -> impl Iterator<Item = Graph::NodeIndex>;
}

pub trait GraphBase {
    type NodeIndex;
}

pub trait WalkableGraph: GraphBase {}

trait NodeWalkExt<Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>:
    NodeWalk<Graph, NodeSubwalk>
{
    fn walk_nodes(&self) -> Vec<Graph::NodeIndex> {
        self.walk().collect()
    }
}

impl<T: NodeWalk<Graph, NodeSubwalk>, Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>
    NodeWalkExt<Graph, NodeSubwalk> for T {}

fn main() {}