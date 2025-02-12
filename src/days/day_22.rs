use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let secret_numbers = read_file();
    match secret_numbers {
        Ok(secret_numbers) => {
            part1(&secret_numbers);
            part2(&secret_numbers);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_next_secret_number(mut num: u64) -> u64 {
    num = (num * 64) ^ num;
    num %= 16777216;
    num = (num / 32) ^ num;
    num %= 16777216;
    num = (num * 2048) ^ num;
    num %= 16777216;
    num
}

fn part1(secret_numbers: &Vec<u64>) {
    let mut sum: u64 = 0;

    for secret in secret_numbers.iter() {
        let mut num = *secret;
        for _ in 0..2000 {
            num = get_next_secret_number(num);
        }
        sum += num;
    }

    println!("Part 1 Answer: {}", sum);
}

fn part2(secret_numbers: &Vec<u64>) {
    let mut patterns_to_price_map: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();
    let mut pattern: (i8, i8, i8, i8) = (0, 0, 0, 0);

    for num in secret_numbers {
        let mut observed_patterns: HashSet<(i8, i8, i8, i8)> = HashSet::new();

        let mut sequence = vec![*num];
        for _ in 0..2000 {
            sequence.push(get_next_secret_number(*sequence.last().unwrap()));
        }

        pattern.0 = (sequence[1] % 10) as i8 - (sequence[0] % 10) as i8;
        pattern.1 = (sequence[2] % 10) as i8 - (sequence[1] % 10) as i8;
        pattern.2 = (sequence[3] % 10) as i8 - (sequence[2] % 10) as i8;
        pattern.3 = (sequence[4] % 10) as i8 - (sequence[3] % 10) as i8;

        *patterns_to_price_map.entry(pattern).or_insert_with(|| 0) += sequence[4] % 10;
        observed_patterns.insert(pattern);

        for i in 5..sequence.len() {
            (pattern.0, pattern.1, pattern.2) = (pattern.1, pattern.2, pattern.3);
            pattern.3 = (sequence[i] % 10) as i8 - (sequence[i - 1] % 10) as i8;

            if !observed_patterns.contains(&pattern) {
                *patterns_to_price_map.entry(pattern).or_insert_with(|| 0) += sequence[i] % 10;
                observed_patterns.insert(pattern);
            }
        }
    }

    let mut max: u64 = 0;
    for key in patterns_to_price_map.keys() {
        if *patterns_to_price_map.get(key).unwrap() > max {
            max = *patterns_to_price_map.get(key).unwrap();
        }
    }
    println!("Part 2 Answer: {}", max);
}

fn read_file() -> Result<Vec<u64>> {
    let mut secret_numbers: Vec<u64> = Vec::new();
    let file = File::open(format!("{}/inputs/day_22.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        secret_numbers.push(line.parse::<u64>().unwrap());
    }
    Ok(secret_numbers)
}
