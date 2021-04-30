use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

const ID_LENGTH: u32 = 160;

type ByteString = [u8; ID_LENGTH];

#[derive(Debug, Serialize, Deserialize)]
pub struct nodeID {
    id: ByteString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node<T> {
    node_id: nodeID,
    ip_addr: IpAddr,
    port: String,
    data: T,
}

impl nodeID {
    fn getNew() -> Self {}
}
