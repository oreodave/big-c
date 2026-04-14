// permit dead code when not using clippy
#![cfg_attr(not(clippy), allow(dead_code))]

mod card;
mod exactsizearr;
mod helper;
mod modes;
mod zipcartesian;

fn main() {
    println!("Hello, world!");
}
