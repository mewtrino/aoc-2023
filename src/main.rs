#![warn(dead_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    day_2();
}

fn day_input(day: u8) -> String {
    let mut input_file = File::open(Path::new(&format!("input/day{}", day))).unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();
    input
}

fn day_1_part_1() {
    let input = day_input(1);
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
    let input = day_input(1);
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

fn day_2() {
    let games = day_input(2);
    let mut valid_game_sum = 0;
    let mut power_sum = 0;
    let (max_red, max_green, max_blue) = (12, 13, 14);
    for (game_n, game) in games.lines().enumerate() {
        let (mut actual_red, mut actual_green, mut actual_blue) = (0, 0, 0);
        if let Some((_, game)) = game.split_once(": ") {
            for sub_game in game.split("; ") {
                for color_num in sub_game.split(", ") {
                    if let Some((num, color)) = color_num.split_once(" ") {
                        let num = num.parse::<i32>().unwrap();
                        match color {
                            "red" => {
                                if num > actual_red {
                                    actual_red = num;
                                }
                            },
                            "green" => {
                                if num > actual_green {
                                    actual_green = num;
                                }
                            },
                            "blue" => {
                                if num > actual_blue {
                                    actual_blue = num;
                                }
                            },
                            _ => (),
                        }
                    }
                }
            }
        }
        if max_red >= actual_red && max_green >= actual_green && max_blue >= actual_blue {
            valid_game_sum += game_n + 1;
        }
        power_sum += actual_red * actual_green * actual_blue;
    }
    println!("Day 2 Part 1:{}", valid_game_sum);
    println!("Day 2 Part 2:{}", power_sum);
}