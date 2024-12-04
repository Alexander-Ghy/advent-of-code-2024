
#[allow(dead_code)]
const PUZZLE_DATA: &str = "puzzle_data.txt";
#[allow(dead_code)]
const DATA: &str = "data.txt";

use std::{fs, io};

fn main() -> io::Result<()> {
    let matrix = read_matrix(DATA);
    //print_matrix(&matrix);
    q1(&matrix);
    q2(&matrix);
    Ok(())
}

fn read_matrix(filename: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(filename).expect("Failed to read file");

    input.lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect()
}

enum Direction {
    North(isize, isize),
    NorthEast(isize, isize),
    East(isize, isize),
    SouthEast(isize, isize),
    South(isize, isize),
    SouthWest(isize, isize),
    West(isize, isize),
    NorthWest(isize, isize),
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::North(-1, 0),
            Direction::NorthEast(-1, 1),
            Direction::East(0, 1),
            Direction::SouthEast(1, 1),
            Direction::South(1, 0),
            Direction::SouthWest(1, -1),
            Direction::West(0, -1),
            Direction::NorthWest(-1, -1),
        ]
    }

    fn delta(&self) -> (isize, isize) {
        match *self {
            Direction::North(dx, dy) => (dx, dy),
            Direction::NorthEast(dx, dy) => (dx, dy),
            Direction::East(dx, dy) => (dx, dy),
            Direction::SouthEast(dx, dy) => (dx, dy),
            Direction::South(dx, dy) => (dx, dy),
            Direction::SouthWest(dx, dy) => (dx, dy),
            Direction::West(dx, dy) => (dx, dy),
            Direction::NorthWest(dx, dy) => (dx, dy),
        }
    }
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<String>>) {
    for row in matrix {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn q1(matrix: &Vec<Vec<String>>)
{
    let mut count = 0;
    let word = "XMAS";
    let word_len = word.len();
    let directions = Direction::all();

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for direction in &directions {
                let (dx, dy) = direction.delta();
                let mut found = true;
                for k in 0..word_len {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;
                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        found = false;
                        break;
                    }
                    if matrix[x as usize][y as usize] != word.chars().nth(k).unwrap().to_string() {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    println!("Q1: {count}");
}

fn q2(matrix: &Vec<Vec<String>>) {
    let mut count = 0;

    println!("Q2: {count}");
}

