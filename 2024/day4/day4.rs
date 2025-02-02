use std::fs::File;
use std::io::BufRead;
use ndarray::Array2;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
fn main() { 
    println!("Part 1 result: {}", part1());

    println!("Part 2 result: {}", part2());
}

fn read_file(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Could not open file");
    let data = std::io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    data
}

// --------------------------------- PART 1 ----------------------------------------------------

fn part1() -> i32{
    let grid = read_file("input.txt");

    let characters = "XMAS".chars().collect::<Vec<char>>();
    
    let rows = grid.len();
    let cols = grid[0].len();
    let data = grid.into_iter().flatten().collect::<Vec<char>>();
    let grid = Array2::from_shape_vec((rows, cols), data).unwrap();

    let mut count: i32 = 0;

  
    for i in 0..rows as i32 { 
        for j in 0..cols as i32 {
            if grid[[i as usize, j as usize]] == characters[0] {
                for &(dx, dy) in &DIRECTIONS {
                    if dfs(&grid, i, j, dx, dy, &characters, 1) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn dfs(
    grid: &Array2<char>,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    target: &[char],
    index: usize
) -> bool {
    if index == target.len() {
        return true;
    }

    let new_x = x + dx;
    let new_y = y + dy;
    
    // bounds
    if  new_x < 0 
        || new_y < 0
        || new_x >= grid.nrows() as i32
        || new_y >= grid.ncols() as i32 
        || grid[[new_x as usize, new_y as usize]] != target[index] {
            return false;
    }

    dfs(grid, new_x, new_y, dx, dy, target, index + 1)
}

// --------------------------------- PART 2 ----------------------------------------------------


fn part2() -> i32{
    let grid = read_file("input.txt");
    
    let rows = grid.len();
    let cols = grid[0].len();
    let data = grid.into_iter().flatten().collect::<Vec<char>>();
    let grid = Array2::from_shape_vec((rows, cols), data).unwrap();
    let rows = grid.nrows() as i32;
    let cols = grid.ncols() as i32;

    let mut count: i32 = 0;

    let mas = ['M', 'A', 'S'];
    let sam = ['S', 'A', 'M'];
    let combinations = [(&mas, &mas), (&sam , &sam), (&mas, &sam), (&sam, &mas)];

    for i in 0..rows{
        for j in 0..cols{
            if grid[[i as usize, j as usize]] == 'A' {
                // define diganoal coordinates
                let diag1 = [(i - 1, j - 1), (i, j), (i + 1, j + 1)];
                let diag2 = [(i - 1, j + 1), (i, j), (i + 1, j - 1)];

                // conditions for diagonals to be inbound
                let diag1_inbound = diag1.iter().all(|&(x, y)| x >= 0 && y >= 0 && x < rows && y < cols);
                let diag2_inbound = diag2.iter().all(|&(x, y)| x >= 0 && y >= 0 && x < rows && y < cols);

                if diag1_inbound && diag2_inbound {
                    // collect characters in diagonals
                    let chars_in_diag1 = diag1
                        .iter()
                        .map(|&(x, y)| grid[[x as usize, y as usize]])
                        .collect::<Vec<char>>();

                    let chars_in_diag2 = diag2
                        .iter()
                        .map(|&(x, y)| grid[[x as usize, y as usize]])
                        .collect::<Vec<char>>();

                    // check if char arrays correnspond to any of our combinations
                    for &(word_diag1, word_diag2) in &combinations {
                        if chars_in_diag1 == word_diag1 && chars_in_diag2 == word_diag2 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}