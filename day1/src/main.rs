

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

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        column1.push(parts[0]);
        column2.push(parts[1]);
    }

    column1.sort();
    column2.sort();

    let mut sum = 0;
    for (a, b) in column1.iter().zip(column2.iter()) {
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        sum += (a - b).abs();
    }

    println!("Q1: {}", sum);
}

fn q2(filename: &str){
    let data = std::fs::read_to_string(filename).expect("Unable to read file");

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        column1.push(parts[0]);
        column2.push(parts[1]);
    }

    let mut similarityscore = 0;
    for i in 0..column1.len() {
        let mut count = 0;
        for j in 0..column2.len() {
            if column1[i] == column2[j] {
                count += 1;
            }
        }
        similarityscore += column1[i].parse::<i32>().expect("not a valid number") * count;
    }

    println!("Q2: {}", similarityscore);
}
