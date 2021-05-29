mod kademlia_dht;
mod node;
mod rpc;
mod trie;

use kademlia_dht::kade_init;
use lib::test;
use node::{ByteString, Node};
use rpc::{handle_message, init_client, init_server};
use std::env;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use trie::RouteTable;

pub fn main() {
    // test();
    // let trie: Trie<i32> = Trie::empty_new();
    // println!("{:?}", trie);

    // let node = ByteString::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    // let mut count = 0;
    // for i in node {
    //     print!("{} ", i);
    //     if count % 8 == 7 {
    //         println!()
    //     }
    //     count += 1
    // }

    // let routetable: RouteTable = RouteTable::empty_new();
    // println!("{:?}", routetable);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provid a server/client and a port");
    }
    let addr = format!("0.0.0.0:{}", args.get(2).unwrap());
    let addr_2 = addr.clone();

    if args.get(1).unwrap() == "client" {
        println!(
            "Your Kademlia DHT Node Client is binded to port: {}",
            args.get(2).unwrap()
        );
        rpc::init_client(&addr);
    } else if args.get(1).unwrap() == "server" {
        println!(
            "Your Kademlia DHT Node Server is binded to port: {}",
            args.get(2).unwrap()
        );
        let rpc = thread::spawn(move || rpc::init_server(&addr));

        kademlia_dht::kade_init(&addr_2);
        rpc.join().unwrap();
    } else {
        println!("None");
    }
}
