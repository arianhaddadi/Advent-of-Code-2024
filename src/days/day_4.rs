use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

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

fn char_at(line: &String, i: usize) -> char {
    line.chars().nth(i).unwrap()
}

fn char_merge_4(a: char, b: char, c: char, d: char) -> String {
    format!("{}{}{}{}", &a, &b, &c, &d)
}

fn part1(lines: &Vec<String>) {
    let height = lines.len();
    let width = lines[0].len();
    let mut total = 0;
    for i in 0..height {
        for j in 0..width {
            if char_at(&lines[i], j) == 'X' {
                // East
                if j < width - 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i], j + 1),
                        char_at(&lines[i], j + 2),
                        char_at(&lines[i], j + 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // North East
                if j < width - 3 && i >= 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i - 1], j + 1),
                        char_at(&lines[i - 2], j + 2),
                        char_at(&lines[i - 3], j + 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // North
                if i >= 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i - 1], j),
                        char_at(&lines[i - 2], j),
                        char_at(&lines[i - 3], j),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // North West
                if j >= 3 && i >= 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i - 1], j - 1),
                        char_at(&lines[i - 2], j - 2),
                        char_at(&lines[i - 3], j - 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // West
                if j >= 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i], j - 1),
                        char_at(&lines[i], j - 2),
                        char_at(&lines[i], j - 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // South West
                if i < height - 3 && j >= 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i + 1], j - 1),
                        char_at(&lines[i + 2], j - 2),
                        char_at(&lines[i + 3], j - 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // South
                if i < height - 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i + 1], j),
                        char_at(&lines[i + 2], j),
                        char_at(&lines[i + 3], j),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }

                // South East
                if i < height - 3 && j < width - 3 {
                    if char_merge_4(
                        char_at(&lines[i], j),
                        char_at(&lines[i + 1], j + 1),
                        char_at(&lines[i + 2], j + 2),
                        char_at(&lines[i + 3], j + 3),
                    ) == "XMAS"
                    {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("Part 1 Answer: {}", total);
}

fn part2(lines: &Vec<String>) {
    let height = lines.len();
    let width = lines[0].len();
    let mut total = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if char_at(&lines[i], j) == 'A' {
                let falling_arm = format!(
                    "{}A{}",
                    char_at(&lines[i - 1], j - 1),
                    char_at(&lines[i + 1], j + 1)
                );
                let rising_arm = format!(
                    "{}A{}",
                    char_at(&lines[i + 1], j - 1),
                    char_at(&lines[i - 1], j + 1)
                );
                if (falling_arm == "MAS" || falling_arm == "SAM")
                    && (rising_arm == "MAS" || rising_arm == "SAM")
                {
                    total += 1;
                }
            }
        }
    }
    println!("Part 2 Answer: {}", total);
}

fn read_file() -> Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();

    let file = File::open(format!("{}/inputs/day_4.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}
