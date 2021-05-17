use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;
use std::rc::Rc;

const NODES_PER_LEAF: usize = 1;
const k: usize = 2;

#[derive(Debug)]
pub enum Vert<T> {
    Vertex(Vertex<T>),
    Leaf(Leaf<T>),
}

type leaf_node<T> = Option<Box<Vert<T>>>;
type node_list<T> = Option<Rc<RefCell<Vec<Node<T>>>>>;

#[derive(Debug)]
pub struct Trie<T> {
    pub length: u64,
    root: leaf_node<T>,
}

#[derive(Debug)]
pub struct Vertex<T> {
    bit: Bit,
    left: leaf_node<T>,
    right: leaf_node<T>,
}

impl<T> Vertex<T> {
    fn new(bit: Bit) -> Vertex<T> {
        Vertex {
            bit,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct Leaf<T> {
    bit: Bit,
    nodes: node_list<T>,
}

impl<T> Leaf<T> {
    pub fn empty_new(bit: Bit) -> Self {
        Leaf {
            bit,
            nodes: Some(Rc::new(RefCell::new(Vec::with_capacity(k)))),
        }
    }

    pub fn add_node(&mut self, node: Node<T>) {
        // TODO
        // Add a single Node to k-bucket vector list
        // let mut node_list = &self.nodes.unwrap();
    }
}

impl<T> Trie<T> {
    pub fn empty_new() -> Self {
        Trie {
            length: 0,
            root: None,
        }
    }
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
