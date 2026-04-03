// permit dead code when not using clippy
#![cfg_attr(not(clippy), allow(dead_code))]

mod card;
mod game;
mod helper;
mod modes;

fn main() {
    println!("Hello, world!");
}
