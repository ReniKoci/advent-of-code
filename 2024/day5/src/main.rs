use std::fs::File;
use std::io::BufRead;
use std::collections::{HashSet, HashMap};

fn main() {
    println!("{}", part1());
}

fn part1()  -> u32{
    let map = get_map("test.txt");
    let files: Vec<String> = get_files("test.txt");
    let mut count: u32 = 0;

    for line in files {
        let numbers = line.split(",")
            .filter_map(|number| number.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        // declare hashset for collecting files that should not come afterwards
        let mut pls_no: HashSet<i32> = HashSet::new();
        let mut flag: bool = true;
        for number in &numbers {
            if pls_no.contains(&number) {
                flag = false;
                println!("{}", &number);
                break;
            }

            if map.contains_key(&number) {
                if let Some(set) = map.get(&number) {
                    for val in set {
                        pls_no.insert(*val);
                    }
                }
            }
        }
        if flag {
            count += numbers[numbers.len() / 2] as u32;
        }
    }

    count
}

fn get_files(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Couldn't read file");
    let reader = std::io::BufReader::new(file);

    let lines = reader.lines()
        .filter_map(|line| line.ok())
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>();

    lines
}

fn get_map(filename: &str) -> HashMap<i32, HashSet<i32>> {
    let file = File::open(filename).expect("Couldn't read file");
    let reader = std::io::BufReader::new(file);

    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Coulnd't read line");
        // read only the first part of the file
        if line == "" {
            break;
        }


        let values = line
            .split("|")
            .filter_map(|l| l.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        map.entry(values[1]).or_insert_with(HashSet::new).insert(values[0]);
    }

    map
}