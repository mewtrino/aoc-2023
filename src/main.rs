#![warn(dead_code)]

use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;

fn main() {
    
}
fn get_puzzle_input(day: u8) -> String {
    let path = Path::new(&format!("./input/day{}", day));
    let mut puzzle_string: String = "";
    let mut file = File::open(&path).expect("Couldn't open input file.");
    file.read_to_string(&mut puzzle_string);
    puzzle_string
}