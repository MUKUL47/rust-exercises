use std::{*, process::exit, io::Write, borrow::Borrow, ops::Deref};
// mod account_usage;
// mod stack;
// mod tictactoe;
mod todo;

fn main(){
    todo::Todo::new().start();
}