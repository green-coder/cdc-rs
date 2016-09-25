extern crate cdc_rs;

use cdc_rs::*;

type IntHash = u32;

static mut hash_id: IntHash = 0;
fn get_new_hash_id() -> IntHash {
    unsafe {
        let id = hash_id;
        hash_id += 1;
        id
    }
}

fn my_new_node(level: usize, children: &Vec<IntHash>) -> Node<IntHash> {
    Node {
        hash: get_new_hash_id(),
        level: level,
        children: children.clone()
    }
}

fn main() {
    let levels = [0usize, 0, 1, 0, 1, 1, 2, 1, 0, 1, 0, 1];
    //let levels = [1usize, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1];
    let hashed_chunk_it = levels.iter().enumerate().map(|(index, level)| {
        HashedChunk {hash: index as IntHash, level: *level}
    });

    unsafe {
        hash_id = levels.len() as IntHash;
    }

    for node in NodeIter::new(hashed_chunk_it, my_new_node, 0) {
        println!("{:?}", node);
    }
}
