use crate::node::{Node, ID_LENGTH};

const NODES_PER_LEAF: usize = 1;
#[derive(Debug)]
pub enum Bit {
    Zero,
    One,
}

#[derive(Debug)]
pub enum Vert<T> {
    Vertex(Vertex<T>),
    Leaf(Leaf<T>),
}

#[derive(Debug)]
pub struct Trie<T> {
    left: Option<Box<Vert<T>>>,
    right: Option<Box<Vert<T>>>,
}

#[derive(Debug)]
pub struct Vertex<T> {
    bit: Bit,
    left: Option<Box<Vert<T>>>,
    right: Option<Box<Vert<T>>>,
}

#[derive(Debug)]
pub struct Leaf<T> {
    bit: Bit,
    nodes: [Node<T>; NODES_PER_LEAF],
}

impl<T> Trie<T> {
    pub fn add_leaf(nodes: [Node<T>; NODES_PER_LEAF]) {}
}

pub fn create<T>() -> Trie<T> {
    Trie::<T> {
        left: None,
        right: None,
    }
}
