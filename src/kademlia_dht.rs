use crate::node::{Bit, ByteString, KademNode, Node, ID_LENGTH};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str;

pub fn kade_init(addr: &str) {
    let port_string = addr.to_string();
    // let port_u16 = port_string.parse::<u16>().unwrap();
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let KadeNode: KademNode<i32, i32> = KademNode::new(localhost_v4, 8111);
    println!("Kade Node Initialized!:");
    println!("{:#?}", KadeNode);
}
