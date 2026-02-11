mod vote;
mod types;
mod parser;

use vote::*;
use types::*;
use parser::*;

fn main() {
    println!("Hello, world!");
    let elec: Election = parser("Data_very_long.txt");
}
