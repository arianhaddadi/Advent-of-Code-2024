use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let grid = read_file();
    match grid {
        Ok(grid) => {
            part1(&grid);
            part2(&grid);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn traverse(
    grid: &Vec<Vec<u8>>,
    i: usize,
    j: usize,
    cur_level: u8,
    summits: &mut HashSet<(usize, usize)>,
) -> u32 {
    if grid[i][j] == 9 {
        summits.insert((i, j));
        return 1;
    }

    let mut num_of_trails = 0;

    if i > 0 && (grid[i - 1][j] == cur_level + 1) {
        num_of_trails += traverse(grid, i - 1, j, cur_level + 1, summits);
    }

    if i < grid.len() - 1 && (grid[i + 1][j] == cur_level + 1) {
        num_of_trails += traverse(grid, i + 1, j, cur_level + 1, summits);
    }

    if j > 0 && (grid[i][j - 1] == cur_level + 1) {
        num_of_trails += traverse(grid, i, j - 1, cur_level + 1, summits);
    }

    if j < grid[0].len() - 1 && (grid[i][j + 1] == cur_level + 1) {
        num_of_trails += traverse(grid, i, j + 1, cur_level + 1, summits);
    }

    num_of_trails
}

fn part1(grid: &Vec<Vec<u8>>) {
    let mut num_of_trailheads = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let mut summits: HashSet<(usize, usize)> = HashSet::new();
                traverse(grid, i, j, 0, &mut summits);
                num_of_trailheads += summits.len();
            }
        }
    }

    println!("Part 1 Answer: {}", num_of_trailheads);
}

fn part2(grid: &Vec<Vec<u8>>) {
    let mut num_of_trails = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let mut summits: HashSet<(usize, usize)> = HashSet::new();
                num_of_trails += traverse(grid, i, j, 0, &mut summits);
            }
        }
    }

    println!("Part 2 Answer: {}", num_of_trails);
}

fn read_file() -> Result<Vec<Vec<u8>>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    let file = File::open("../inputs/day_10.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<u8> = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as u8)
            .collect();
        grid.push(digits);
    }

    Ok(grid)
}
