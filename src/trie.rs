use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;

const NODES_PER_LEAF: usize = 1;

#[derive(Debug)]
pub enum Vert<T> {
    Vertex(Vertex<T>),
    Leaf(Leaf<T>),
}

type leaf_node<T> = Option<Box<Vert<T>>>;
type node_list<T> = Option<RefCell<Vec<Node<T>>>>;

#[derive(Debug)]
pub struct Trie<T> {
    pub length: u64,
    left: leaf_node<T>,
    right: leaf_node<T>,
}

#[derive(Debug)]
pub struct Vertex<T> {
    bit: Bit,
    left: leaf_node<T>,
    right: leaf_node<T>,
}

#[derive(Debug)]
pub struct Leaf<T> {
    bit: Bit,
    nodes: node_list<T>,
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Trie {
            length: 0,
            left: None,
            right: None,
        }
    }
    pub fn add_leaf(node: ByteString) {
        for bit in ByteString {
            match bit {}
        }
    }
}
