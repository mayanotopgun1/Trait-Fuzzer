pub trait Sequence<Item, Subsequence: Sequence<Item, Subsequence>> {}

pub trait NodeWalk<Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>:
    Sequence<Graph::NodeIndex, NodeSubwalk>
{
}

pub trait GraphBase {
    type NodeIndex;
}

pub trait WalkableGraph: GraphBase {}

trait NodeWalkExt<Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>:
    NodeWalk<Graph, NodeSubwalk>
{
    fn walk_nodes(&self) -> Vec<Graph::NodeIndex> {
        // Dummy implementation for demonstration
        vec![]
    }
}

impl<T: NodeWalk<Graph, NodeSubwalk>, Graph: GraphBase, NodeSubwalk: NodeWalk<Graph, NodeSubwalk>>
    NodeWalkExt<Graph, NodeSubwalk> for T {}

fn main() {}