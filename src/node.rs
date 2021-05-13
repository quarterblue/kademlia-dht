use rand::random;
use std::net::IpAddr;

pub const ID_LENGTH: usize = 20;

#[derive(Debug)]
pub struct ByteString(pub [u8; ID_LENGTH]);

#[derive(Debug)]
pub enum Bit {
    Zero,
    One,
    None,
}

impl ByteString {
    pub fn get(index: usize) -> Option<Bit> {
        // todo
        None
    }
}

#[derive(Debug)]
pub struct nodeID {
    id: ByteString,
}

#[derive(Debug)]
pub struct Node<T> {
    node_id: nodeID,
    ip_addr: IpAddr,
    port: u16,
    data: T,
}

impl nodeID {
    pub fn new() -> Self {
        let mut node = nodeID { id: [0; ID_LENGTH] };
        for i in 0..ID_LENGTH {
            node.id[i] = random::<u8>();
        }
        return node;
    }
}
