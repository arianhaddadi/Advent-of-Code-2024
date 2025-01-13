use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let lines = read_file();
    match lines {
        Ok((results, numbers)) => {
            part1(&results, &numbers);
            part2(&results, &numbers);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn is_valid(expected_result: u64, numbers: &Vec<u64>, operators_vec: &Vec<char>) -> bool {
    let mut real_result = numbers[0];

    for i in 0..operators_vec.len() {
        if operators_vec[i] == '+' {
            real_result += numbers[i + 1];
        } else if operators_vec[i] == '*' {
            real_result *= numbers[i + 1];
        } else {
            // Concatenation Operator
            let num_of_digits = numbers[i + 1].to_string().len() as u32;
            real_result = real_result * u64::pow(10, num_of_digits) + numbers[i + 1];
        }
    }

    real_result == expected_result
}
fn traverse(
    result: u64,
    numbers: &Vec<u64>,
    operators_vec: &mut Vec<char>,
    depth: usize,
    is_concat_operator_available: bool,
) -> bool {
    if depth == operators_vec.len() {
        return if is_valid(result, numbers, operators_vec) {
            true
        } else {
            false
        };
    }

    operators_vec[depth] = '+';
    if traverse(
        result,
        numbers,
        operators_vec,
        depth + 1,
        is_concat_operator_available,
    ) {
        return true;
    }

    operators_vec[depth] = '*';
    if traverse(
        result,
        numbers,
        operators_vec,
        depth + 1,
        is_concat_operator_available,
    ) {
        return true;
    }

    if is_concat_operator_available {
        operators_vec[depth] = '|';
        if traverse(
            result,
            numbers,
            operators_vec,
            depth + 1,
            is_concat_operator_available,
        ) {
            return true;
        }
    }

    false
}

fn part1(results: &Vec<u64>, numbers: &Vec<Vec<u64>>) {
    let num_of_cases = results.len();
    let mut sum_can_become_true = 0;
    for i in 0..num_of_cases {
        let mut operators_vec = vec![' '; numbers[i].len() - 1];
        if traverse(results[i], &numbers[i], &mut operators_vec, 0, false) {
            sum_can_become_true += results[i];
        }
    }

    println!("Part 1 Answer: {}", sum_can_become_true);
}

fn part2(results: &Vec<u64>, numbers: &Vec<Vec<u64>>) {
    let num_of_cases = results.len();
    let mut sum_can_become_true = 0;
    for i in 0..num_of_cases {
        let mut operators_vec = vec![' '; numbers[i].len() - 1];
        if traverse(results[i], &numbers[i], &mut operators_vec, 0, true) {
            sum_can_become_true += results[i];
        }
    }

    println!("Part 2 Answer: {}", sum_can_become_true);
}

fn read_file() -> Result<(Vec<u64>, Vec<Vec<u64>>)> {
    let mut results: Vec<u64> = Vec::new();
    let mut numbers: Vec<Vec<u64>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_7.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let line: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        assert_eq!(line.len(), 2);

        results.push(line[0].parse::<u64>().unwrap());
        numbers.push(
            line[1]
                .split(' ')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect(),
        );
    }

    Ok((results, numbers))
}
