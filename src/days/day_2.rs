use std::{fs::File, io::BufRead, io::BufReader};

pub fn run() {
    let mut lines = read_file();
    let mut lines_clone = lines.clone();
    part1(&mut lines);
    part2(&mut lines_clone);
}

fn is_safe(line: &mut Vec<u32>, i: u32, mut tolerate: bool) -> bool {
    let i: usize = i as usize;

    if i == (line.len() - 1) {
        return true;
    }

    let diff = line[i + 1] as i32 - line[i] as i32;
    if diff < 1 || diff > 3 {
        if !tolerate {
            return false;
        }
        let cache = line[i];
        if i > 0 {
            line[i] = line[i - 1];
            if is_safe(line, i as u32, false) {
                return true;
            }
        } else {
            if is_safe(line, (i + 1) as u32, false) {
                return true;
            }
        }
        line[i + 1] = cache;
        tolerate = false;
    }
    return is_safe(line, (i + 1) as u32, tolerate);
}

fn part1(lines: &mut Vec<Vec<u32>>) {
    let mut num_safes = 0;
    for line in lines {
        if is_safe(&mut line.clone(), 0, false) {
            num_safes += 1;
        } else {
            line.reverse();
            if is_safe(line, 0, false) {
                num_safes += 1;
            }
        }
    }
    println!("Part 1 Answer: {}", num_safes);
}

fn part2(lines: &mut Vec<Vec<u32>>) {
    let mut num_safes = 0;
    for line in lines {
        if is_safe(&mut line.clone(), 0, true) {
            num_safes += 1;
        } else {
            line.reverse();
            if is_safe(line, 0, true) {
                num_safes += 1;
            }
        }
    }
    println!("Part 2 Answer: {}", num_safes);
}

fn read_file() -> Vec<Vec<u32>> {
    let mut lines: Vec<Vec<u32>> = Vec::new();
    let file = File::open("../inputs/day_2.txt");
    if let Ok(file) = file {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(
                    line.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect(),
                );
            }
        }
    }
    lines
}
