#![warn(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    day_1_part_2();
}

fn day_1_part_1() {
    let mut input_file = File::open(Path::new("input/day1"))
        .expect("Failed to open day 1 file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Failed to read day 1 file");
    let mut sum = 0;
    for line in input.lines() {
        let mut nums = Vec::new();
        for char in line.chars() {
            if char.is_ascii_digit() {
                nums.push(char)
            }
        }
        let num = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        sum += num.parse::<i32>().unwrap();
    }
    println!("{}",sum);
}

fn day_1_part_2() {
    let mut input_file = File::open(Path::new("input/day1"))
        .expect("Failed to open day 1 file");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Failed to read day 1 file");
    let mut sum = 0;
    let digits = [ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    for line in input.lines() {
        let mut nums = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        for (line_index, char) in line.char_indices() {
            if char.is_ascii_digit() {
                nums.push(char.to_string())
            }
            for (digit_index, digit) in digits.iter().enumerate() {
                let search_string = &chars[line_index..line.len()];
                if (search_string.iter().collect::<String>()).starts_with(digit) {
                    // index is numerical value of digit because of how we set up array digits
                    nums.push(digit_index.to_string());
                }
            }
        }
        let num = format!("{}{}", nums.first().unwrap(), nums.last().unwrap());
        sum += num.parse::<i32>().unwrap();
    }
    println!("Day 1 Part 2:{}", sum);
}