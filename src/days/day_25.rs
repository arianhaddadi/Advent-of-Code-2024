use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let file_content = read_file();
    match file_content {
        Ok((keys, locks)) => {
            part1(&keys, &locks);
            part2();
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn key_lock_match(key: &Vec<usize>, lock: &Vec<usize>) -> bool {
    assert_eq!(key.len(), lock.len());
    for i in 0..key.len() {
        if key[i] + lock[i] > 7 {
            return false;
        }
    }
    true
}

fn part1(keys: &Vec<Vec<usize>>, locks: &Vec<Vec<usize>>) {
    let mut num_of_pairs = 0;
    for i in 0..keys.len() {
        for j in 0..locks.len() {
            if key_lock_match(&keys[i], &locks[j]) {
                num_of_pairs += 1;
            }
        }
    }
    println!("Part 1 Answer: {}", num_of_pairs);
}

fn part2() {
    println!("Part 2 Answer: The chronicle is finished. Hooray!");
}

fn get_numerical_form(grid: Vec<Vec<char>>) -> Vec<usize> {
    let mut numerical_form: Vec<usize> = Vec::new();

    for col in 0..grid[0].len() {
        let mut num = 0;
        for row in 0..grid.len() {
            if grid[row][col] == '.' {
                continue;
            }
            num += 1;
        }
        numerical_form.push(num);
    }

    numerical_form
}

fn read_file() -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();

    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_25.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if grid[0][0] == '.' {
                keys.push(get_numerical_form(grid));
            } else {
                locks.push(get_numerical_form(grid));
            }
            grid = Vec::new();
            continue;
        }

        let digits: Vec<char> = line.chars().collect();
        grid.push(digits)
    }

    if grid[0][0] == '.' {
        keys.push(get_numerical_form(grid));
    } else {
        locks.push(get_numerical_form(grid));
    }

    Ok((keys, locks))
}
