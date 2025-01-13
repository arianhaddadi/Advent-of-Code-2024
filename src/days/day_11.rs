use core::result::Result::Err;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Stone {
    num: u64,
    remaining_blinks: u8,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Stone(num: {}, remaining_blinks: {})",
            self.num, self.remaining_blinks
        )
    }
}

pub fn run() {
    let nums = read_file();
    match nums {
        Ok(nums) => {
            part1(get_stones(&nums, 25));
            part2(get_stones(&nums, 75));
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_stones(nums: &Vec<u64>, num_of_blinks: u8) -> Vec<Stone> {
    let mut stones: Vec<Stone> = Vec::new();
    for num in nums {
        stones.push(Stone {
            num: *num,
            remaining_blinks: num_of_blinks,
        })
    }
    stones
}

fn get_num_of_digits(num: u64) -> usize {
    return num.to_string().len();
}

fn has_even_num_of_digits(num: u64) -> bool {
    get_num_of_digits(num) % 2 == 0
}

fn divide_stone(stone: &mut Stone) -> Stone {
    let num_str = stone.num.to_string();
    let mut new_stone = stone.clone();
    stone.num = num_str[0..num_str.len() / 2].parse::<u64>().unwrap();
    new_stone.num = num_str[(num_str.len() / 2)..].parse::<u64>().unwrap();
    new_stone.remaining_blinks -= 1;
    new_stone
}

fn blink(mut stone: Stone, results_storage: &mut HashMap<Stone, usize>) -> usize {
    if results_storage.contains_key(&stone) {
        return *results_storage.get(&stone).unwrap();
    }

    let initial_stone = stone;
    let mut num_of_stones = 1;
    while stone.remaining_blinks > 0 {
        if stone.num == 0 {
            stone.num += 1;
        } else if has_even_num_of_digits(stone.num) {
            let new_stone = divide_stone(&mut stone);
            num_of_stones += blink(new_stone, results_storage)
        } else {
            stone.num *= 2024;
        }
        stone.remaining_blinks -= 1;
    }

    results_storage.insert(initial_stone, num_of_stones);
    num_of_stones
}

fn part1(stones: Vec<Stone>) {
    let mut results_storage: HashMap<Stone, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut num_of_stones: usize = 0;
    while i < stones.len() {
        if stones[i].remaining_blinks > 0 {
            num_of_stones += blink(stones[i], &mut results_storage);
        }
        i += 1;
    }
    println!("Part 1 Answer: {}", num_of_stones);
}

fn part2(stones: Vec<Stone>) {
    let mut results_storage: HashMap<Stone, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut num_of_stones: usize = 0;
    while i < stones.len() {
        if stones[i].remaining_blinks > 0 {
            num_of_stones += blink(stones[i], &mut results_storage);
        }
        i += 1;
    }
    println!("Part 2 Answer: {}", num_of_stones);
}

fn read_file() -> Result<Vec<u64>> {
    let mut nums: Vec<u64> = Vec::new();

    let file = File::open("../inputs/day_11.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        nums = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
    }

    Ok(nums)
}
