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

#[derive(Debug)]
pub enum Route {
    RouteTable(String),
    None,
}

impl ByteString {
    pub fn random_new() -> Self {
        let mut node = [0; ID_LENGTH];
        for i in 0..ID_LENGTH {
            node[i] = random::<u8>();
        }
        ByteString(node)
    }

    pub fn get(index: usize) -> Option<Bit> {
        // todo
        None
    }
}

#[derive(Debug)]
pub struct Node {
    node_id: ByteString,
    ip_addr: IpAddr,
    port: u16,
    route_table: Route,
}

pub trait KadeNode {
    fn get_ip() -> IpAddr;
    fn get_port() -> u16;
    fn get_nodeid() -> ByteString;
}

impl Node {
    pub fn new_node(ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id: ByteString::random_new(),
            ip_addr,
            port,
            route_table: Route::None,
        }
    }

    pub fn new_self(ip_addr: IpAddr, port: u16) -> Self {
        Node {
            node_id: ByteString::random_new(),
            ip_addr,
            port,
            route_table: Route::None,
        }
    }
}
