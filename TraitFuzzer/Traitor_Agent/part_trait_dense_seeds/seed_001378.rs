struct Node<C: Trait>(C::Assoc::<Self>);

trait Trait {
    type Assoc<T>;
}

impl Trait for Vec<()> {
    type Assoc<T> = Vec<T>;
}

trait NodeTrait {
    fn new_node() -> Self;
}

impl<C: Trait> NodeTrait for Node<C>
where
    C::Assoc<Self>: Default,
{
    fn new_node() -> Self {
        Node(Default::default())
    }
}

fn main() {
    let _ = Node::<Vec<()>>::new_node();
}