#![allow(unused)]

use std::env;

mod actions;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    println!("{:?}", args)
}
