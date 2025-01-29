use std::fs;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

fn main() {
    let (mut pq1, mut pq2) = get_pq("input.txt");
    let mut res: i64 = 0;


    // Solution to part 1
    while let (Some(val1), Some(val2)) = (pq1.pop(), pq2.pop()) {
        res += (val1 - val2).abs() as i64;
    }

    println!("Number is {}", res);


    // Solution to part 2
    let (mut pq1, mut pq2) = get_pq("input.txt");
    let mut res: i64 = 0;
    while let Some(val1) = pq1.pop() { 
        let mut count : i64 = 0;
        
        while let Some(&top) = pq2.peek() {
            if top > (val1 as i64) {
                pq2.pop();
            } else {
                break;
            }
        }

        while let Some(&top) = pq2.peek() {
            if top == (val1 as i64) {
                pq2.pop();
                count += 1;
            } else {
                break;
            }
        }
        res += count * val1;
    }

    println!("Number is {}", res);

}

fn get_pq(filename: &str) -> (BinaryHeap<i64>, BinaryHeap<i64>) {
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut pq1 = BinaryHeap::new();
    let mut pq2 = BinaryHeap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect();

        if numbers.len() == 2 {
            pq1.push(numbers[0]);
            pq2.push(numbers[1]);
        } else {
            panic!("Error")
        }
    }

    (pq1, pq2)
}