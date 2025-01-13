use std::{
    collections::HashMap,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let result = read_file();
    match result {
        Ok((rules, updates)) => {
            let children = extract_rules(&rules);
            part1(&children, &updates);
            part2(&children, &updates);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn extract_rules(rules: &Vec<String>) -> HashMap<u32, HashSet<u32>> {
    let mut children: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules {
        let parts: Vec<u32> = rule
            .split('|')
            .map(|s: &str| s.parse::<u32>().unwrap())
            .collect();
        assert_eq!(parts.len(), 2);

        children
            .entry(parts[0])
            .or_insert_with(HashSet::new)
            .insert(parts[1]);

        children.entry(parts[1]).or_insert_with(HashSet::new);
    }
    children
}

fn is_valid(children: &HashMap<u32, HashSet<u32>>, update_vec: &mut Vec<u32>, swap: bool) -> bool {
    for i in 0..update_vec.len() {
        for j in i + 1..update_vec.len() {
            if children
                .get(&update_vec[j])
                .unwrap()
                .contains(&update_vec[i])
            {
                if swap {
                    (update_vec[i], update_vec[j]) = (update_vec[j], update_vec[i]);
                }
                return false;
            }
        }
    }
    true
}

fn part1(children: &HashMap<u32, HashSet<u32>>, updates: &Vec<String>) {
    let mut total = 0;
    for update in updates {
        let mut update_vec: Vec<u32> = update
            .split(',')
            .map(|s: &str| s.parse::<u32>().unwrap())
            .collect();
        if is_valid(children, &mut update_vec, false) {
            total += update_vec[update_vec.len() / 2];
        }
    }
    println!("Part 1 Answer: {}", total);
}

fn part2(children: &HashMap<u32, HashSet<u32>>, updates: &Vec<String>) {
    let mut total = 0;
    for update in updates {
        let mut update_vec: Vec<u32> = update
            .split(',')
            .map(|s: &str| s.parse::<u32>().unwrap())
            .collect();
        if !is_valid(children, &mut update_vec, true) {
            while !is_valid(children, &mut update_vec, true) {}
            total += update_vec[update_vec.len() / 2];
        }
    }
    println!("Part 2 Answer: {}", total);
}

fn read_file() -> Result<(Vec<String>, Vec<String>)> {
    let mut rules: Vec<String> = Vec::new();
    let mut updates: Vec<String> = Vec::new();

    let file = File::open(format!("{}/inputs/day_5.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    let mut is_reading_rules = true;
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            // We have reached the separator between rules and updates
            is_reading_rules = false;
            continue;
        }
        if is_reading_rules {
            rules.push(line);
        } else {
            updates.push(line);
        }
    }

    Ok((rules, updates))
}
