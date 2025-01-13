use std::{fs::File, io::BufRead, io::BufReader, io::Result};

pub fn run() {
    let lines = read_file();
    match lines {
        Ok(lines) => {
            part1(&lines);
            part2(&lines);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn traverse(line: &String, mut i: usize) -> Option<u32> {
    let mut first_num_str = String::new();
    let mut second_num_str = String::new();
    i += 4;

    if line.chars().nth(i)? == ',' {
        return None;
    }

    // Phase 1 (First Num)
    while i < line.len() && line.chars().nth(i)? >= '0' && line.chars().nth(i)? <= '9' {
        first_num_str.push(line.chars().nth(i)?);
        i += 1;
    }
    if i >= line.len() || line.chars().nth(i)? != ',' {
        return None;
    }

    i += 1;
    if line.chars().nth(i)? == ')' {
        return None;
    }

    // Phase 2 (Second Num)
    while i < line.len() && line.chars().nth(i)? >= '0' && line.chars().nth(i)? <= '9' {
        second_num_str.push(line.chars().nth(i)?);
        i += 1;
    }

    if i >= line.len() || line.chars().nth(i)? != ')' {
        return None;
    }

    Some(first_num_str.parse::<u32>().unwrap() * second_num_str.parse::<u32>().unwrap())
}

fn part1(lines: &Vec<String>) {
    let mut total = 0;
    for line in lines {
        let mut i = 0;
        while i < (line.len() - 7) as usize {
            if &line[i..i + 4] == "mul(" {
                let res = traverse(&line, i);
                if let Some(res) = res {
                    total += res;
                }
                i += 3;
            }
            i += 1;
        }
    }
    println!("Part 1 Answer: {}", total);
}

fn part2(lines: &Vec<String>) {
    let mut total = 0;
    let mut enabled = true;
    for line in lines {
        let mut i = 0;
        while i < (line.len() - 7) as usize {
            if &line[i..i + 4] == "do()" {
                enabled = true;
                i += 4;
                continue;
            }

            if &line[i..i + 7] == "don't()" {
                enabled = false;
                i += 7;
                continue;
            }

            if &line[i..i + 4] == "mul(" {
                if !enabled {
                    i += 4;
                    continue;
                }
                let res = traverse(&line, i);
                if let Some(res) = res {
                    total += res;
                }
                i += 3;
            }
            i += 1;
        }
    }
    println!("Part 2 Answer: {}", total);
}

fn read_file() -> Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();

    let file = File::open(format!("{}/inputs/day_3.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
