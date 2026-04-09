#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ops::Deref;

struct Root {
    jsref: JSRef
}

impl Deref for Root {
    type Target = JSRef;

    fn deref<'a>(&'a self) -> &'a JSRef {
        &self.jsref
    }
}

#[derive(Copy, Clone)]
struct JSRef {
    node: *const Node
}

impl Deref for JSRef {
    type Target = Node;

    fn deref<'a>(&'a self) -> &'a Node {
        self.get()
    }
}

trait INode {
    fn RemoveChild(&self, usize);
    fn AddChild(&self, usize);
}

impl INode for JSRef {
    fn RemoveChild(&self, _a: usize) {
        self.get().RemoveChild(_a)
    }

    fn AddChild(&self, _a: usize) {
        self.get().AddChild(_a);
    }
}

impl JSRef {
    fn get<'a>(&'a self) -> &'a Node {
        unsafe {
            &*self.node
        }
    }
}

struct Node;

impl Node {
    fn RemoveChild(&self, _a: usize) {
    }

    fn AddChild(&self, _a: usize) {
    }
}

trait JSRefExt: INode {}

impl<T: INode> JSRefExt for T {}

fn create_node_ref(n: &Node) -> JSRef {
    JSRef { node: n as *const Node }
}

fn main() {
    let n = Node;
    let jsref = create_node_ref(&n);
    let root = Root { jsref };

    root.AddChild(0);
    jsref.AddChild(0);

    root.RemoveChild(0);
    jsref.RemoveChild(0);
}