use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;
use std::rc::Rc;

const NODES_PER_LEAF: usize = 1;
const K_BUCKET_SIZE: usize = 4; // Optimal K_BUCKET_SIZE is 20, for testing purposes, use 4

#[derive(Debug)]
pub enum Vert {
    Vertex(Vertex),
    Leaf(Leaf),
}

type leaf_node = Option<Box<Vertex>>;
type node_list = Option<Rc<RefCell<Vec<Node>>>>;

pub struct KBucket {
    node_bucket: NodeBucket,
}

impl KBucket {
    pub fn new() -> Self {
        KBucket {
            node_bucket: NodeBucket::new(),
        }
    }
}

pub struct NodeBucket {
    node_vector: Vec<Node>,
}

impl NodeBucket {
    pub fn new() -> Self {
        NodeBucket {
            node_vector: Vec::new(),
        }
    }
    pub fn sort() {}
    pub fn get_latest() -> Node {
        // TODO
    }
    pub fn get_oldest() -> Node {
        // TODO
    }
}

#[derive(Debug)]
pub struct Trie {
    pub length: u64,
    root: leaf_node,
}

#[derive(Debug)]
pub struct Vertex {
    bit: Bit,
    k_bucket: Vec<Node>,
    left: leaf_node,
    right: leaf_node,
}

impl<T> Vertex<T> {
    fn new(bit: Bit) -> Vertex<T> {
        Vertex {
            bit,
            k_bucket: Vec::with_capacity(K_BUCKET_SIZE),
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct Leaf<T> {
    bit: Bit,
    nodes: node_list,
}

impl<T> Leaf<T> {
    pub fn empty_new(bit: Bit) -> Self {
        Leaf {
            bit,
            nodes: Some(Rc::new(RefCell::new(Vec::with_capacity(k)))),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        // TODO
        // Add a single Node to k-bucket vector list
        // let mut node_list = &self.nodes.unwrap();
    }
}

// Trie is the implementation of the routing table composed of k-buckets
impl<T> Trie<T> {
    pub fn empty_new() -> Self {
        Trie {
            length: 0,
            root: Some(Box::new(Vertex::new(Bit::None))),
        }
    }

    // Split k_bucket when length > k_bucket_size
    pub fn split() {}

    // Add vertex to the trie that contains k_bucket
    pub fn add_vertex() {}

    // Add node to the k_bucket
    pub fn add_node() {}

    pub fn add_leaf(&mut self, node: ByteString) {
        match &self.root {
            Some(value) => {
                // Bytes
                for i in 0..20 {
                    // Bits
                    for j in 0..8 {
                        if (i == 19 && j == 7) {
                            // Handle leaf case
                            let leaf: Option<Box<Vert<T>>> =
                                Some(Box::new(Vert::Leaf(Leaf::empty_new(Bit::One))));
                        } else {
                            // Handle vertex case
                            if (node.0[i] >> j) & 1 == 1 {
                                let leaf_tmp: Option<Box<Vert<T>>> =
                                    Some(Box::new(Vert::Vertex(Vertex::new(Bit::One))));
                            } else {
                                let leaf_tmp: Option<Box<Vert<T>>> =
                                    Some(Box::new(Vert::Vertex(Vertex::new(Bit::Zero))));
                            }
                        }
                    }
                }
            }
            None => {
                let mut root: Option<Box<Vert<T>>> =
                    Some(Box::new(Vert::Vertex(Vertex::new(Bit::None))));
                self.root = root;
            }
        }
    }

    pub fn add_leafrec(&mut self, add_node: Vertex<T>, node: ByteString) {
        match add_node.bit {
            Bit::One => {
                println!("Handle One Case")
            }
            Bit::Zero => {
                println!("Handle Two Case")
            }
            _ => {
                println!("Something went wrong")
            }
        }
    }
}
