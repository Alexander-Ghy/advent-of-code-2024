#[allow(dead_code)]
const PUZZLE_DATA: &str = "puzzle_data.txt";
#[allow(dead_code)]
const DATA: &str = "data.txt";

fn main() {
    q1(PUZZLE_DATA);
    q2(PUZZLE_DATA);
}

fn q1(filename: &str) {
    let data = std::fs::read_to_string(filename).expect("Unable to read file");

    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        lines.push(parts.iter().map(|x| x.parse::<i32>().unwrap()).collect());
    }

    //println!("{:?}", lines);

    let mut sum = 0;
    for line in &lines {
        if is_vector_decreasing_or_increasing(line) && difference_between_values(line) {
            //println!("{:?}", line);
            sum += 1;
        }
    }

    println!("Q1: {}", sum);
}

fn is_vector_decreasing_or_increasing(v: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            decreasing = false;
        } else if v[i] < v[i-1] {
            increasing = false;
        }
    }
    increasing || decreasing
}

fn difference_between_values(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if (v[i] - v[i-1]).abs() > 3 || (v[i] - v[i-1]).abs() < 1 {
            return false;
        }
    }
    true
}

fn q2(filename: &str){
    let data = std::fs::read_to_string(filename).expect("Unable to read file");

    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        lines.push(parts.iter().map(|x| x.parse::<i32>().unwrap()).collect());
    }

    //println!("{:?}", lines);

    let mut sum = 0;
    for line in &lines {
        if is_vector_decreasing_or_increasing(line) && difference_between_values(line) {
            //println!("{:?}", line);
            sum += 1;
        }
        else {
            for i in 0..line.len() { // remove one element and check if it is valid -> is so, then break
                let mut modified_line = line.clone();
                modified_line.remove(i);
                if is_vector_decreasing_or_increasing(&modified_line) && difference_between_values(&modified_line) {
                    //println!("{:?}", modified_line);
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("Q2: {}", sum);
}
