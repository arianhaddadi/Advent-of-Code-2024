use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let cols = read_file();
    if let Some(mut cols) = cols {
        part1(&mut cols);
        part2(&mut cols);
    }
}

fn part1(cols: &mut (Vec<i32>, Vec<i32>)) {
    let (ref mut col1, ref mut col2) = cols;
    col1.sort();
    col2.sort();

    let size = col1.len();
    let mut distance = 0;
    for i in 0..size {
        distance += (col1[i] - col2[i]).abs();
    }

    println!("Part 1 Answer: {}", distance);
}

fn part2(cols: &mut (Vec<i32>, Vec<i32>)) {
    let mut reps: HashMap<i32, i32> = HashMap::new();
    let (ref mut col1, ref mut col2) = cols;
    for value in col2 {
        *reps.entry(*value).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for value in col1 {
        if reps.contains_key(&value) {
            similarity += *value * reps[&value];
        }
    }

    println!("Part 2 Answer: {}", similarity);
}

fn read_file() -> Option<(Vec<i32>, Vec<i32>)> {
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    let file = File::open("../inputs/day_1.txt");
    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let (Ok(val1), Ok(val2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    column1.push(val1);
                    column2.push(val2);
                } else {
                    eprintln!("Error parsing integers in line: {}", line);
                    return None;
                }
            }
        }

        return Some((column1, column2));
    }
    return None;
}
