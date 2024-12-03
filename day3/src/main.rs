
#[allow(dead_code)]
const PUZZLE_DATA: &str = "puzzle_data.txt";
#[allow(dead_code)]
const DATA: &str = "data.txt";

use std::{fs, io};
use regex::Regex;

fn main() -> io::Result<()> {
    q1(PUZZLE_DATA);
    q2(PUZZLE_DATA);
    Ok(())
}

fn q1(filename: &str)
{
    let input = fs::read_to_string(filename).expect("Failed to read file");
    let regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let mut sum = 0;
    for m in regex.find_iter(&input) {
        let values = m.as_str().replace("mul(", "").replace(")", "");
        let (val1, val2) = values.split_once(',').unwrap();
        sum += val1.parse::<u32>().unwrap() * val2.parse::<u32>().unwrap()
    }

    println!("Q1: {sum}");
}

fn q2(filename: &str)
{
    let input = fs::read_to_string(filename).expect("Failed to read file");
    let regex = Regex::new(r"(mul\(\d+,\d+\)|do(n't)?\(\))").unwrap();

    let mut do_next = true;
    let mut sum = 0;
    for m in regex.find_iter(&input) {
        match m.as_str() {
            "do()" => do_next = true,
            "don't()" => do_next = false,
            mul if do_next => {
                let values = mul.replace("mul(", "").replace(")", "");
                let (val1, val2) = values.split_once(',').unwrap();
                sum += val1.parse::<u32>().unwrap() * val2.parse::<u32>().unwrap()
            }
            _ => {}
        }
    }

    println!("Q2: {sum}");
}