mod vote;
mod types;
mod parser;

use vote::*;
use types::*;

fn main() {
    println!("Hello, world!");
    let elec: Election = parse("Data_very_long.txt");
}