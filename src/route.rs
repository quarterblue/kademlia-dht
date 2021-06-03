use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;
use std::iter::Iterator;

use std::rc::Rc;

const NODES_PER_LEAF: usize = 1;
const K_BUCKET_SIZE: usize = 4; // Optimal K_BUCKET_SIZE is 20, for testing purposes, use 4

type LeafNode = Option<Rc<RefCell<Vertex>>>;
type node_list = Option<Rc<RefCell<Vec<Node>>>>;

#[derive(Debug)]
pub struct KBucket {
    node_bucket: Vec<Node>,
    depth: usize,
}

impl KBucket {
    pub fn new() -> Self {
        KBucket {
            node_bucket: Vec::with_capacity(K_BUCKET_SIZE),
            depth: 0,
        }
    }
    pub fn sort(&mut self) {}
    // pub fn get_latest(&self) -> &Node<K, V> {
    //     // TODO
    // }
    // pub fn get_oldest(&self) -> &Node<K, V> {
    //     // TODO
    // }

    fn split(&self) -> (Option<KBucket>, Option<KBucket>) {
        let mut left = KBucket::new();
        let mut right = KBucket::new();
        left.depth = self.depth + 1;
        right.depth = self.depth + 1;

        for node in &self.node_bucket {
            match node.node_id.index(self.depth + 1) {
                0 => left.node_bucket.push(*node),
                1 => right.node_bucket.push(*node),
                _ => unreachable!(),
            }
        }

        (Some(left), Some(right))
    }
}

// Represents a single vertex in the trie of the Route Table.
// This vertex could have a k-bucket, in which case it is a leaf.
// If the vertex does not have a k-bucket, it is a inner vertex.
#[derive(Debug)]
pub struct Vertex {
    bit: Bit,
    k_bucket: Option<KBucket>,
    parent: LeafNode,
    left: LeafNode,
    right: LeafNode,
}

impl Vertex {
    fn new(bit: Bit) -> Vertex {
        Vertex {
            bit,
            k_bucket: Some(KBucket::new()),
            parent: None,
            left: None,
            right: None,
        }
    }

    // Creates 2 vertices and splits the current k-bucket and instantiates 2 new k-buckets
    // with the correseponding nodes
    fn split(&mut self) -> (Option<Rc<RefCell<Vertex>>>, Option<Rc<RefCell<Vertex>>>) {
        // Allocate two new vertices for left and right
        let mut left = Vertex::new(Bit::Zero);
        let mut right = Vertex::new(Bit::One);
        // Split the buckets into tuple
        let tuple = self.k_bucket.as_ref().unwrap().split();
        // Deallocate the current bucket
        self.k_bucket = None;
        // Link the new k-buckets to left and right vertices
        left.k_bucket = tuple.0;
        right.k_bucket = tuple.1;

        (
            Some(Rc::new(RefCell::new(left))),
            Some(Rc::new(RefCell::new(right))),
        )
    }

    // Recursively adds a node to the current vertex by finding the closest matching k-bucket
    fn add_node<I: Iterator<Item = u8>>(&mut self, node: Node, node_iter: &mut I) {
        match &mut self.k_bucket {
            Some(x) => {
                // Check that K-Bucket is not full, add node to the bucket
                if x.node_bucket.len() < K_BUCKET_SIZE {
                    x.node_bucket.push(node);
                } else {
                    // Since K-Bucket is full, split the K-Bucket into two if K-Bucket
                    // and retry adding to the current node
                    let (left_vert, right_vert) = self.split();
                    self.left = left_vert;
                    self.right = right_vert;
                    self.add_node(node, node_iter);
                }
            }
            None => {
                // This is a vertex with no k-bucket, trickle down to Left or Right vertex
                // Depending on the iter.next() bit
                match node_iter.next().unwrap() {
                    0 => match &self.left {
                        Some(vert) => {
                            vert.borrow_mut().add_node(node, node_iter);
                        }
                        None => {}
                    },
                    1 => match &self.right {
                        Some(vert) => {
                            vert.borrow_mut().add_node(node, node_iter);
                        }
                        None => {}
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }

    fn add_k(&mut self, node: Node) {}
}

// Trie structure representing the Route table composed of k-buckets
#[derive(Debug)]
pub struct RouteTable {
    pub length: u64,
    root: LeafNode,
}

// Implementation of the routing table composed of k-buckets
impl RouteTable {
    pub fn empty_new() -> Self {
        RouteTable {
            length: 0,
            root: Some(Rc::new(RefCell::new(Vertex::new(Bit::None)))),
        }
    }

    // Add vertex to the trie that contains k_bucket
    pub fn add_vertex() {}

    // Add a node to the k_bucket starting from the root of the trie
    pub fn add_node(&mut self, node: Node) {
        match self.root.as_mut() {
            Some(x) => {
                let mut iter = node.node_id.into_iter();
                x.borrow_mut().add_node(node, &mut iter);
                self.length += 1;
            }
            None => {
                // Root does not exist, Error handling
                panic!("Root does not exist");
            }
        }
    }

    fn find_closest(&self, node_id: [u8; ID_LENGTH]) -> Vec<Node> {
        let alpha_nodes: Vec<Node> = Vec::new();
        match self.root {
            Some(ref x) => match &x.borrow_mut().k_bucket {
                Some(bucket) => {}
                None => {}
            },
            None => {}
        }
        return alpha_nodes;
    }
}