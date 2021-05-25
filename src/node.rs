use rand::random;
use std::collections::HashMap;
use std::iter::Iterator;
use std::net::IpAddr;

use crate::trie::RouteTable;

pub const ID_LENGTH: usize = 20;
#[derive(Debug)]
pub enum Bit {
    Zero,
    One,
    None,
}

#[derive(Debug)]
pub struct ByteString(pub [u8; ID_LENGTH], usize);

impl ByteString {
    pub fn new(arr: [u8; ID_LENGTH]) -> Self {
        ByteString(arr, 0)
    }
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
        // todo
        None
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

#[derive(Debug)]
pub struct Node<K, V> {
    node_id: ByteString,
    ip_addr: IpAddr,
    port: u16,
    route_table: Option<RouteTable<K, V>>,
    hash_map: Option<HashMap<K, V>>,
}

pub trait KadeNode {
    fn get_ip() -> IpAddr;
    fn get_port() -> u16;
    fn get_nodeid() -> ByteString;
}

pub trait RPC<K, V> {
    fn find_node() -> Node<K, V>;
    fn store() -> bool;
    fn find_value(key: K) -> V;
    fn ping() -> bool;
}

impl<K, V> Node<K, V> {
    pub fn new_node(ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id: ByteString::random_new(),
            ip_addr,
            port,
            route_table: None,
            hash_map: None,
        }
    }

    pub fn new_self(ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id: ByteString::random_new(),
            ip_addr,
            port,
            route_table: None,
            hash_map: Some(HashMap::new()),
        }
    }
}
