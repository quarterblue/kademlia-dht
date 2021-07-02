use bincode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::{thread, time};
pub enum Request<K, V> {
    Ping,
    Store(K, V),
    FindNode,
    FindValue(K, V),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum RequestFuncs {
    Ping,
    Store(i32),
    FindNode(i32),
    FindValue(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ResponseFuncs {
    Ping(bool),
    Store(bool),
    FindNode(i32),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Message {
    Request(RequestFuncs),
    Response(ResponseFuncs),
}

pub struct MessageO {
    demux_key: u8,
    sender_id: u8,
    payload: Option<u8>,
}

pub fn serialize_message(msg: Message) -> Vec<u8> {
    bincode::serialize(&msg).expect("Could not serialize message")
}

pub fn deserialize_message(v: Vec<u8>) -> Message {
    bincode::deserialize(&v).expect("Could not serialize message")
}

pub fn handle_message(sock: UdpSocket, size: usize, src: SocketAddr, buf: [u8; 1500]) {
    let mut vec = buf.to_vec();
    vec.resize(size, 0);
    let msg = deserialize_message(vec);
    let mut resp = ResponseFuncs::Error("Unknown request".to_string());

    if let Message::Request(msg) = msg {
        resp = match msg {
            RequestFuncs::Ping => {
                // TODO Handle Ping
                println!("Ping!");
                ResponseFuncs::Ping(true)
            }
            RequestFuncs::Store(s) => {
                // TODO Handle Store
                println!("Store: {}", s);
                ResponseFuncs::Store(true)
            }
            RequestFuncs::FindNode(s) => {
                // TODO Handle Find Node
                println!("Store: {}", s);
                ResponseFuncs::Store(true)
            }
            RequestFuncs::FindValue(s) => {
                // TODO Handle Find Value
                println!("Store: {}", s);
                ResponseFuncs::Store(true)
            }
        }
    }

    let resp_msg = Message::Response(resp);
    let serialized = serialize_message(resp_msg);

    sock.send_to(&serialized, &src)
        .expect("Failed to send a resposne");
}
pub fn req_rep(sock: UdpSocket, req: RequestFuncs) -> ResponseFuncs {
    let msg = Message::Request(req);
    let serialized = serialize_message(msg);

    sock.send(&serialized).expect("Failed to write to server");
    let mut buf = [0u8; 1500];

    let (len, _src) = sock
        .recv_from(&mut buf)
        .expect("Could not read into buffer");

    let resp = deserialize_message(buf.to_vec());
    if let Message::Response(resp) = resp {
        return resp;
    }

    return ResponseFuncs::Error("No valid response".to_string());
}

pub fn init_server(addr: &str) {
    let socket = UdpSocket::bind(addr).expect("Could not bind socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((sz, src)) => {
                thread::spawn(move || {
                    handle_message(sock, sz, src, buf);
                });
            }
            Err(e) => {
                eprintln!("couldn't receive datagram: {}", e);
            }
        }
    }
}

pub fn init_client(addr: &str) {
    let socket = UdpSocket::bind(addr).expect("Could not bind client socket");
    socket
        .connect("127.0.0.1:8111")
        .expect("Could not connect to server");

    let mut i = 0;
    loop {
        let fmt = format!("Hello Iteration {}", i);
        let resp = req_rep(
            socket.try_clone().expect("Could not clone socket"),
            RequestFuncs::Ping,
        );

        println!("{:?}", resp);

        let resp = req_rep(
            socket.try_clone().expect("Could not clone socket"),
            RequestFuncs::Store(32),
        );
        println!("{:?}", resp);
        i += 1;
        thread::sleep(time::Duration::from_millis(500));
    }
}
