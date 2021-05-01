mod node;
mod trie;
use lib::test;
use trie::create;

pub fn main() {
    test();
    let trie = create::<i32>();
    println!("{:?}", trie);
}
