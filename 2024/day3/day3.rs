use std::fs;
use std::collections::HashMap;

fn main() {
    println!("{}", part1("input.txt"));
}

fn part1(filename: &str) -> i32 {
    let text: String = fs::read_to_string(filename).expect("Error");
    let mut result: i32 = 0;

    // match mut( and find start index
    let v: Vec<_> = text.match_indices("mul(").collect();
    
    // match "do"
    let dos: Vec<_> = text.match_indices("do").collect();
    
    // match "don't"
    let donts: Vec<_> = text.match_indices("don't").collect();

    // add dos and donts into hashmap <index, string>
    // add indices in vector
    let mut rules = HashMap::new();
    let mut indices = Vec::new();
    for d in dos {
        rules.insert(d.0, d.1);
        indices.push(d.0);
    }

    for d in donts {
        rules.insert(d.0, d.1);
        indices.push(d.0);
    }

    //sort
    indices.sort();

    for idk in v {
        let index = idk.0 + 4; // shift 4 to go to number
        let do_or_dont = binary_search(&indices, index);
        
        if do_or_dont == 0  {
            let slice = &text[index..];
            result += get_numbers(slice);
        } else {
            println!("{}", do_or_dont);
            let rule = match rules.get(&indices[do_or_dont]) {
                Some(&child) => child, // If the rule exists, use it
                None => "unknown",      // If the rule doesn't exist, default to "unknown"
            };

            if rule == "do" {
                let slice = &text[index..];
                result += get_numbers(slice);
            }
        }
    }

    result
} 

fn get_numbers(text: &str) -> i32 {

    if let Some((end_index, _)) = text.char_indices().find(|&(_,c)| c == ')') {
        // we have start and end index. 
        let substring: &str = &text[..end_index];

        // Check for "," to split
        let parts: Vec<&str> = substring.split(",").collect();

        // if there are more or less than 2 parts, return
        if parts.len() != 2 {
            return 0;
        }

        if parts[0].chars().all(|c| c.is_numeric()) && parts[1].chars().all(|c| c.is_numeric()){
            return multiply(parts[0], parts[1]);
        } else {
            return 0;
        }

    }
    
    return 0;
    
}

fn multiply(number1: &str, number2: &str) -> i32{
    if let (Ok(num1), Ok(num2)) = (number1.parse::<i32>(), number2.parse::<i32>()) {
        return num1 * num2;
    } 

    0
}

fn binary_search(indices: &Vec<usize>, target: usize) -> usize {
    // check whether target is in bound of the first rule
    if target < indices[0] {
        return 0;
    }

    if target > indices[indices.len() - 1] {
        return indices.len() - 1;
    }
    
    let mut left: usize = 0;
    let mut right: usize = indices.len() - 1;
    while left <= right {
        let mid: usize = (left + right) / 2;
        if target == indices[mid] {
            return mid;
        } else if target < indices[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    // println!("{} - {} - target: {}", indices[left], indices[right], target);
    right
}