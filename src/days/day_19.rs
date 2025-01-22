use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let file_content = read_file();
    match file_content {
        Ok((towels, designs)) => {
            part1(&towels, &designs);
            part2(&towels, &designs);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_possible_towels<'a>(
    towels: &'a Vec<String>,
    index: usize,
    design: &String,
) -> Vec<&'a String> {
    let mut possible_towels: Vec<&String> = Vec::new();
    for towel in towels.iter() {
        if (design.len() - index >= towel.len()) && (*towel == design[index..index + towel.len()]) {
            possible_towels.push(towel);
        }
    }
    possible_towels
}

fn traverse(
    towels: &Vec<String>,
    design: &String,
    index: usize,
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    if index == design.len() {
        return 1;
    }
    if cache.contains_key(&index) {
        return *cache.get(&index).unwrap();
    }
    let mut num_possible = 0;
    let possible_towels = get_possible_towels(towels, index, design);
    for possible_towel in possible_towels {
        num_possible += traverse(towels, design, index + possible_towel.len(), cache);
    }
    cache.insert(index, num_possible);
    num_possible
}

fn part1(towels: &Vec<String>, designs: &Vec<String>) {
    let mut num_possible = 0;
    let mut cache: HashMap<usize, u64> = HashMap::new();
    for design in designs.iter() {
        cache.clear();
        if traverse(towels, design, 0, &mut cache) > 0 {
            num_possible += 1;
        }
    }
    println!("Part 1 Answer: {}", num_possible);
}

fn part2(towels: &Vec<String>, designs: &Vec<String>) {
    let mut num_possible = 0;
    let mut cache: HashMap<usize, u64> = HashMap::new();
    for design in designs.iter() {
        cache.clear();
        num_possible += traverse(towels, design, 0, &mut cache);
    }
    println!("Part 2 Answer: {}", num_possible);
}

fn read_file() -> Result<(Vec<String>, Vec<String>)> {
    let mut towels: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();
    let file = File::open(format!("{}/inputs/day_19.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);
    let mut towel_phase = true;

    for line in reader.lines() {
        let line = line?;
        if towel_phase {
            towels = line
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            towel_phase = false;
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        designs.push(line);
    }
    Ok((towels, designs))
}
