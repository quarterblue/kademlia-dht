# Kademlia Distributed Hash Table
[![Rust](https://github.com/quarterblue/kademlia-dht/actions/workflows/rust.yml/badge.svg)](https://github.com/quarterblue/kademlia-dht/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
<img src=https://img.shields.io/github/last-commit/quarterblue/kademlia-dht></img>

<img width="40%" src="https://upload.wikimedia.org/wikipedia/commons/6/63/Dht_example_SVG.svg" alt="kademlia">

Kademlia distributed hash table is a peer-to-peer dht with provable consistency and performance in a fault-prone environment. It is based on XOR distance metric topology to facilitate the communication between nodes.

This is a standalone implementation of the Kademlia distributed table in pure Rust. This implementation tries to follow the original paper as close as possible. Refer to the <a href="https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf">original paper</a> for more information.

**Other references:**

- <a href="https://pub.tik.ee.ethz.ch/students/2006-So/SA-2006-19.pdf">Implementation of Kademlia by Bruno Spori</a>

- <a href="http://maude.sip.ucm.es/kademlia/files/pita_kademlia.pdf">Specification of the Kademlia DHT by Isabel Pita</a>

## Installation

To run a server, clone the repo and run
```bash
$ cargo run server 8111
```

To run a client, clone the repo and run
```bash
$ cargo run client 8222
```

To run a full node, clone the repo and run
```bash
$ cargo run full 8111
```

## Usage

```rust
// To be updated
```
