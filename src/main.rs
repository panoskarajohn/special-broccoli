// use std::{path::Path, fs::File, io::{BufReader, BufRead}};
// use aoc::search::search::{get_first_number_from_text, get_last_number_from_text};

use aoc::dayone::dayone::{dayone};

fn main() {
    let path = get_path_from_env();
    dayone(&path); // Replace or remove this line
    // let path = get_path_from_env();
    // let inputs = read_input(&path).unwrap();
    // let sum = get_sum_from_input(&inputs);
    // println!("The total sum is {}", sum);
}

fn get_path_from_env() -> String {
    let arg = std::env::args().nth(1).expect("no argument given");
    return arg;
}


