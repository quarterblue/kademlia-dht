mod kademlia_dht;
mod node;
mod route;
mod rpc;

use kademlia_dht::kade_init;
use lib::test;
use node::{ByteString, Node};
use route::RouteTable;
use rpc::{handle_message, init_client, init_server};
use std::env;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide a server/client and a port");
    }
    let addr = format!("0.0.0.0:{}", args.get(2).unwrap());
    let addr_2 = addr.clone();

    let node_type = &args.get(1).unwrap()[..];

    match node_type {
        "client" => {
            println!(
                "Your Kademlia DHT Node Client is binded to port: {}",
                args.get(2).unwrap()
            );
            rpc::init_client(&addr);
        }
        "server" => {
            println!(
                "Your Kademlia DHT Node Server is binded to port: {}",
                args.get(2).unwrap()
            );
            let rpc = thread::spawn(move || rpc::init_server(&addr));

            kademlia_dht::kade_init(&addr_2);
            rpc.join().unwrap();
        }
        _ => {
            println!("You must provide a valid type (server or client).");
        }
    }
}
