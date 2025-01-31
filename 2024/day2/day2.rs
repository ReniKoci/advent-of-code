use std::fs;
use std::io::{self, BufRead};

fn main() { 
    let result: i32 = part1("input.txt");
    println!("Result is: {}", result);

    let result: i32 = part2("input.txt");
    println!("Result is: {}", result);
}

fn part2(filename: &str) -> i32 { 
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // read line
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        
        if is_safe(&numbers) {
            result += 1;
            continue; // stop loop
        }

        // check for 1 error sequences
        for i in 0..numbers.len() {
            let mut numbers_clone = numbers.clone();
            numbers_clone.remove(i);

            if is_safe(&numbers_clone) {
                result += 1;
                break;
            }
        }
    }

    result
}


fn part1(filename: &str) -> i32 { 
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // read line
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        
        if is_safe(&numbers) {
            result += 1;
        }
    }

    result
}


fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut safe: bool = true;

    let mut sign: char = '0'; // 0 - not assigned, 1 - positive, 2 - negative
    for i in 1..numbers.len() {
        if (numbers[i] - numbers[i - 1]).abs() > 3 || (numbers[i] - numbers[i - 1]).abs() < 1 {
            safe = false;
            break;
        }

        let current_sign: char;
        if numbers[i] - numbers[i - 1] > 0 {
            current_sign = '1';
        } else {
            current_sign = '2';
        }

        if sign != '0' && sign != current_sign {
            safe = false;
            break;
        }

        if sign == '0' {
            sign = current_sign;
        }
        
    }
    safe
}