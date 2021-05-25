use crate::node::{Bit, ByteString, Node, ID_LENGTH};
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

const NODES_PER_LEAF: usize = 1;
const K_BUCKET_SIZE: usize = 4; // Optimal K_BUCKET_SIZE is 20, for testing purposes, use 4

type leaf_node<K, V> = Option<Box<Vertex<K, V>>>;
type node_list<K, V> = Option<Rc<RefCell<Vec<Node<K, V>>>>>;

#[derive(Debug)]
pub struct KBucket<K, V> {
    node_bucket: Vec<Node<K, V>>,
}

impl<K, V> KBucket<K, V> {
    pub fn new() -> Self {
        KBucket {
            node_bucket: Vec::with_capacity(K_BUCKET_SIZE),
        }
    }
    pub fn sort() {}
    pub fn get_latest() -> Node<K, V> {
        // TODO
    }
    pub fn get_oldest() -> Node<K, V> {
        // TODO
    }
}

#[derive(Debug)]
pub struct Vertex<K, V> {
    bit: Bit,
    k_bucket: Option<KBucket<K, V>>,
    left: leaf_node<K, V>,
    right: leaf_node<K, V>,
}

impl<K, V> Vertex<K, V> {
    fn new(bit: Bit) -> Vertex<K, V> {
        Vertex {
            bit,
            k_bucket: Some(KBucket::new()),
            left: None,
            right: None,
        }
    }

    fn add_node<I: Iterator<Item = u8>>(&mut self, node: Node<K, V>, node_iter: I) {
        match node_iter.next().unwrap() {
            0 => match self.left {
                Some(vert) => {}
                None => {}
            },
            1 => match self.right {
                Some(vert) => {}
                None => {}
            },
        }

        // if self.k_bucket.node_bucket.len() < K_BUCKET_SIZE {
        //     self.k_bucket.node_bucket.push(node);
        // } else {
        //     // Trickle down to next k_bucket
        //     // TO DO
        //     println!("TODO")
        // }
    }
}

// Trie structure representing the Route table composed of K buckets
#[derive(Debug)]
pub struct RouteTable<K, V> {
    pub length: u64,
    root: leaf_node<K, V>,
}

// Trie is the implementation of the routing table composed of k-buckets
impl<K, V> RouteTable<K, V> {
    pub fn empty_new() -> Self {
        RouteTable {
            length: 0,
            root: Some(Box::new(Vertex::new(Bit::None))),
        }
    }

    // Split k_bucket when length > k_bucket_size
    pub fn split() {}

    // Add vertex to the trie that contains k_bucket
    pub fn add_vertex() {}

    // Add node to the k_bucket
    pub fn add_node(&mut self, node: Node<K, V>) {
        match self.root {
            Some(unwrapped) => {
                let vert = *unwrapped;
                if vert.k_bucket.is_none() {
                    // TODO
                    // This is a vertex with no K_Bucket, trickle down to Left or Right vertex
                    // Depending on the iter.next() bit
                } else {
                    // TODO
                    // Check that K_Bucket is not full, add node to the bucket
                    // Split the K_Bucket into two if full
                }
            }
            None => {
                // TODO
                // Error handling
            }
        }
    }
}
