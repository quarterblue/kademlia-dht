mod node;
mod trie;

use lib::test;
use node::{ByteString, Node};
use trie::RouteTable;

pub fn main() {
    // test();
    // let trie: Trie<i32> = Trie::empty_new();
    // println!("{:?}", trie);
    println!("hello world");
    let node = ByteString::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut count = 0;
    for i in node {
        print!("{} ", i);
        if count % 8 == 7 {
            println!()
        }
        count += 1
    }

    let routetable: RouteTable = RouteTable::empty_new();
    println!("{:?}", routetable);
}
