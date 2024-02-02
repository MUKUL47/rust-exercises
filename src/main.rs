use std::{
    borrow::{ Borrow, BorrowMut },
    future::IntoFuture,
    io::{ Write, Cursor },
    ops::{ Deref, DerefMut },
    process::exit,
    thread::sleep,
    time::Duration,
    *,
};
// mod account_usage;
mod stack;
mod tictactoe;
mod todo;
mod trie;
mod hashmap;
mod traits;
#[tokio::main]
async fn main() {
    test().into_future();
}
async fn test() -> i32 {
    return 1;
}
