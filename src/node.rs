use rand::random;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Iterator;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::route::RouteTable;

pub const ID_LENGTH: usize = 20;
#[derive(Debug)]
pub enum Bit {
    Zero,
    One,
    Root,
}

// 160 bit Node ID in tuple, 0 position is an array of bits, and 1 position is the size
#[derive(Debug, Clone, Copy)]
pub struct ByteString(pub [u8; ID_LENGTH], usize);

impl ByteString {
    // ByteString constructor that takes in a preset array of bitstring
    pub fn new(arr: [u8; ID_LENGTH]) -> Self {
        ByteString(arr, 0)
    }

    // ByteString constructor to create an empty node ID
    pub fn new_empty() -> Self {
        let node = [0; ID_LENGTH];
        ByteString(node, 0)
    }

    // ByeString constructor to create a random 160 bit node ID
    pub fn random_new() -> Self {
        let mut node = [0; ID_LENGTH];
        for i in 0..ID_LENGTH {
            node[i] = random::<u8>();
        }
        ByteString(node, 0)
    }

    pub fn index(&self, i: usize) -> u8 {
        let base = i / 8;
        let offset = i % 8;
        (self.0[base] >> offset) & 1
    }

    pub fn get(index: usize) -> Option<Bit> {
        todo!()
    }
}

impl PartialEq for ByteString {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..ID_LENGTH {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        return true;
    }
}

// UNTESTED IMPLEMENTATION
impl PartialOrd for ByteString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..ID_LENGTH {
            if self.0[i].cmp(&other.0[i]) == Ordering::Equal {
                continue;
            } else {
                return Some(self.0[i].cmp(&other.0[i]));
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Iterator for ByteString {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret;
        if self.1 > ID_LENGTH * 8 - 1 {
            ret = None;
        } else {
            ret = Some(self.index(self.1));
        }
        self.1 += 1;
        ret
    }
}
pub trait RPC<K, V> {
    fn find_node(&self) -> Node;
    fn store(&mut self) -> bool;
    fn find_value(&self, key: K) -> V;
    fn ping(&self) -> bool;
}
// KademNode represents the entire data struct of a node
// Contained within is the route table for finding other nodes, and hash map to store the k, v pairs
#[derive(Debug)]
pub struct KademNode<K, V> {
    pub node_id: ByteString,
    pub ip_addr: IpAddr,
    pub port: u16,
    pub route_table: Option<RouteTable>,
    hash_map: Option<HashMap<K, V>>,
}

impl<K, V> KademNode<K, V> {
    pub fn new(ip_addr: IpAddr, port: u16) -> Self {
        let node_id = ByteString::random_new();
        KademNode {
            node_id,
            ip_addr,
            port,
            route_table: Some(RouteTable::empty_new(node_id.clone())),
            hash_map: Some(HashMap::new()),
        }
    }
}
// Node represents a entity that is running a Kademlia DHT protocol
// It can be uniquely identified by the node_id and has ip addr and port for UDP connection
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub node_id: ByteString,
    pub ip_addr: IpAddr,
    pub port: u16,
}

pub trait KadeNode {
    fn get_ip() -> IpAddr;
    fn get_port() -> u16;
    fn get_nodeid() -> ByteString;
}

impl Node {
    // The default new node constructor
    // returns a node with a random 160 bit node_id
    pub fn new_node(ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id: ByteString::random_new(),
            ip_addr,
            port,
        }
    }

    // The Node constructor for testing purposes only
    // returns a node with an empty node id
    // the node id should be manually set and tested for equality and ordering
    pub fn test_node(ip_addr: IpAddr, port: u16, arr: [u8; ID_LENGTH]) -> Self {
        let empty_bytestring = ByteString::new(arr);
        Node {
            node_id: empty_bytestring,
            ip_addr,
            port,
        }
    }

    // Calculates and returns XOR Distance between two nodes
    pub fn distance(&self, node_id: ByteString) -> ByteString {
        let mut nodeid = ByteString::new_empty();
        for i in 0..20 {
            nodeid.0[i] = node_id.0[i] ^ self.node_id.0[i]
        }
        return nodeid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_distance() {
        let new_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let arr: [u8; ID_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
        let port: u16 = 5111;
        let new_node = Node::test_node(new_ip, port, arr);

        let comp_arr: [u8; ID_LENGTH] =
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9];

        let comp_byte = ByteString::new(comp_arr);

        let test_arr: [u8; ID_LENGTH] =
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14];

        let xor_distance = new_node.distance(comp_byte);

        assert_eq!(xor_distance.0, test_arr);
    }

    #[test]
    fn new_node() {
        let new_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let arr: [u8; ID_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
        let port: u16 = 5111;
        let new_node = Node::test_node(new_ip, port, arr);
        assert_eq!(new_node.ip_addr, new_ip);
        assert_eq!(new_node.port, port);
    }

    #[test]
    fn bytestring_eq() {
        let arr_one: [u8; ID_LENGTH] =
            [0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
        let arr_two: [u8; ID_LENGTH] =
            [0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
        let arr_three: [u8; ID_LENGTH] =
            [0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];

        let byte_one = ByteString::new(arr_one);
        let byte_two = ByteString::new(arr_two);
        let byte_three = ByteString::new(arr_three);

        assert_eq!(byte_one, byte_two);
        assert_ne!(byte_one, byte_three);
    }
}
