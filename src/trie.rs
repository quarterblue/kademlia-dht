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
    pub fn sort(&mut self) {}
    pub fn get_latest(&self) -> &Node<K, V> {
        // TODO
    }
    pub fn get_oldest(&self) -> &Node<K, V> {
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
        if self.k_bucket.is_none() {
            // TODO
            // This is a vertex with no K_Bucket, trickle down to Left or Right vertex
            // Depending on the iter.next() bit
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
        } else {
            // TODO
            // Check that K_Bucket is not full, add node to the bucket
            let array = self.k_bucket.unwrap().node_bucket;
            if array.len() < K_BUCKET_SIZE {
                array.push(node);
            } else {
                // Split the K_Bucket into two if K_Bucket is full
            }
        }
    }

    fn add_k(&mut self, node: Node<K, V>) {}
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
    fn add_node<I: Iterator<Item = u8>>(&mut self, node: Node<K, V>, node_iter: I) {
        match self.root {
            Some(unwrapped) => {
                let vert = *unwrapped;
                vert.add_node(node, node_iter);
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
