use crate::node::{Bit, ByteString, KademNode, Node, ID_LENGTH};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str;

pub fn kade_init(addr: &str) {
    let port_string = addr.to_string();
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut kade_node: KademNode<i32, i32> = KademNode::new(localhost_v4, 8111);

    println!("Kade Node Initialized!:");
    println!("{:#?}", &kade_node);
    println!("-------");

    let one_ip = IpAddr::V4(Ipv4Addr::new(28, 2, 9, 1));
    let one_node = Node::new_node(one_ip, 8111);
    kade_node.route_table.as_mut().unwrap().add_node(one_node);

    let two_ip = IpAddr::V4(Ipv4Addr::new(18, 7, 9, 1));
    let two_node = Node::new_node(two_ip, 8111);
    kade_node.route_table.as_mut().unwrap().add_node(two_node);
    println!("{:#?}", &kade_node);
}
