use std::{*, process::exit, io::{Write, Cursor}, borrow::{Borrow, BorrowMut}, ops::{Deref, DerefMut}, thread::sleep, time::Duration,  };
// mod account_usage;
mod stack;
mod tictactoe;
mod todo;
mod trie;
mod hashmap;

fn main(){
    let mut hashmap = hashmap::MyHashMap::new(10);
    hashmap.set("name", "John");
    hashmap.set("naem", "John1");
    hashmap.set("mane", "John3");
    // hashmap.set("age", "25");
    println!("{:?}",hashmap.get("name"));
}
