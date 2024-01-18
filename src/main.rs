use std::{*, process::exit, io::Write, borrow::Borrow, ops::Deref};
mod account_usage;
mod stack;
mod tictactoe;
fn main(){
    tictactoe::TicTacToe::new().start_game()
}
