use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;
use std::iter::Iterator;

use std::rc::Rc;

const NODES_PER_LEAF: usize = 1;
const K_BUCKET_SIZE: usize = 4; // Optimal K_BUCKET_SIZE is 20, for testing purposes, use 4

type leaf_node = Option<Box<Vertex>>;
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
    left: leaf_node,
    right: leaf_node,
}

impl Vertex {
    fn new(bit: Bit) -> Vertex {
        Vertex {
            bit,
            k_bucket: Some(KBucket::new()),
            left: None,
            right: None,
        }
    }

    // TODO
    fn split(&mut self) -> (Option<Box<Vertex>>, Option<Box<Vertex>>) {
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

        (Some(Box::new(left)), Some(Box::new(right)))
    }

    // Recursively adds a node to the current vertex by finding the closest matching k-bucket
    fn add_node<I: Iterator<Item = u8>>(&mut self, node: Node, node_iter: &mut I) {
        if self.k_bucket.is_none() {
            // This is a vertex with no k-bucket, trickle down to Left or Right vertex
            // Depending on the iter.next() bit
            match node_iter.next().unwrap() {
                0 => match self.left {
                    Some(_) => {
                        self.left.as_mut().unwrap().add_node(node, node_iter);
                    }
                    None => {}
                },
                1 => match self.right {
                    Some(_) => {
                        self.right.as_mut().unwrap().add_node(node, node_iter);
                    }
                    None => {}
                },
                _ => {}
            }
        } else {
            // Check that K_Bucket is not full, add node to the bucket
            if self.k_bucket.as_mut().unwrap().node_bucket.len() < K_BUCKET_SIZE {
                self.k_bucket.as_mut().unwrap().node_bucket.push(node);
            } else {
                // TODO
                // Split the K_Bucket into two if K_Bucket is full
            }
        }
    }

    fn add_k(&mut self, node: Node) {}
}

// Trie structure representing the Route table composed of k-buckets
#[derive(Debug)]
pub struct RouteTable {
    pub length: u64,
    root: leaf_node,
}

// Implementation of the routing table composed of k-buckets
impl RouteTable {
    pub fn empty_new() -> Self {
        RouteTable {
            length: 0,
            root: Some(Box::new(Vertex::new(Bit::None))),
        }
    }

    // Add vertex to the trie that contains k_bucket
    pub fn add_vertex() {}

    // Add a node to the k_bucket starting from the root of the trie
    fn add_node(&mut self, node: Node) {
        match self.root.as_mut() {
            Some(_) => {
                let vert = self.root.as_mut().unwrap();
                let mut iter = node.node_id.into_iter();
                vert.add_node(node, &mut iter);
                self.length += 1;
            }
            None => {
                // TODO
                // Root does not exist
                // Error handling
            }
        }
    }
}
