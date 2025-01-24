use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let file_content = read_file();
    match file_content {
        Ok((a_reg, b_reg, c_reg, instructions)) => {
            part1(a_reg, b_reg, c_reg, &instructions);
            part2(b_reg, c_reg, &instructions);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_combo_value(operand: u64, a_reg: &mut u64, b_reg: &mut u64, c_reg: &mut u64) -> u64 {
    if operand <= 3 {
        return operand;
    }
    if operand == 4 {
        return *a_reg;
    }
    if operand == 5 {
        return *b_reg;
    }
    *c_reg
}

fn execute(
    instruction_pointer: &mut usize,
    a_reg: &mut u64,
    b_reg: &mut u64,
    c_reg: &mut u64,
    stdout: &mut String,
    instructions: &Vec<u64>,
) {
    let instruction = instructions[*instruction_pointer];
    let operand = instructions[*instruction_pointer + 1];
    match instruction {
        0 => {
            *a_reg = *a_reg / (2u64.pow(get_combo_value(operand, a_reg, b_reg, c_reg) as u32));
            *instruction_pointer += 2;
        }
        1 => {
            *b_reg ^= operand;
            *instruction_pointer += 2;
        }
        2 => {
            *b_reg = get_combo_value(operand, a_reg, b_reg, c_reg) % 8;
            *instruction_pointer += 2;
        }
        3 => {
            if *a_reg != 0 {
                *instruction_pointer = operand as usize;
            } else {
                *instruction_pointer += 2;
            }
        }
        4 => {
            *b_reg ^= *c_reg;
            *instruction_pointer += 2;
        }
        5 => {
            stdout.push_str(
                ((get_combo_value(operand, a_reg, b_reg, c_reg) % 8).to_string() + ",").as_str(),
            );
            *instruction_pointer += 2;
        }
        6 => {
            *b_reg = *a_reg / (2u64.pow(get_combo_value(operand, a_reg, b_reg, c_reg) as u32));
            *instruction_pointer += 2;
        }
        7 => {
            *c_reg = *a_reg / (2u64.pow(get_combo_value(operand, a_reg, b_reg, c_reg) as u32));
            *instruction_pointer += 2;
        }
        _ => {
            return;
        }
    }
}

fn execute_all(
    mut a_reg: u64,
    mut b_reg: u64,
    mut c_reg: u64,
    instructions: &Vec<u64>,
    from_addr: usize,
    to_addr: usize,
) -> String {
    let mut stdout = String::new();
    let mut instruction_pointer = from_addr;
    while instruction_pointer < to_addr {
        execute(
            &mut instruction_pointer,
            &mut a_reg,
            &mut b_reg,
            &mut c_reg,
            &mut stdout,
            instructions,
        );
    }
    stdout[0..stdout.len() - 1].to_string()
}

fn part1(a_reg: u64, b_reg: u64, c_reg: u64, instructions: &Vec<u64>) {
    println!(
        "Part 1 Answer: {}",
        execute_all(a_reg, b_reg, c_reg, instructions, 0, instructions.len())
    );
}

fn part2(b_reg: u64, c_reg: u64, instructions: &Vec<u64>) {
    let mut possibles: Vec<u64> = vec![0];
    let mut outputs = instructions.clone();
    outputs.reverse();

    for output in outputs.iter() {
        let mut new_possibles: Vec<u64> = Vec::new();
        for possible in possibles.iter() {
            for i in 0..8 {
                let candidate = possible * 8 + i;
                let execution_output = execute_all(
                    candidate,
                    b_reg,
                    c_reg,
                    instructions,
                    0,
                    instructions.len() - 2,
                );
                if output.to_string() == execution_output {
                    new_possibles.push(candidate);
                }
            }
        }
        possibles = new_possibles;
    }

    possibles.sort();
    println!("Part 2 Answer: {}", possibles[0]);
}

fn read_file() -> Result<(u64, u64, u64, Vec<u64>)> {
    let mut a_reg: Option<u64> = None;
    let mut b_reg: Option<u64> = None;
    let mut c_reg: Option<u64> = None;
    let mut instructions: Vec<u64> = Vec::new();

    let file = File::open(format!("{}/inputs/day_17.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        if a_reg.is_none() {
            a_reg = Some(
                line.split(": ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[1]
                    .parse::<u64>()
                    .unwrap(),
            );
            continue;
        }

        if b_reg.is_none() {
            b_reg = Some(
                line.split(": ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[1]
                    .parse::<u64>()
                    .unwrap(),
            );
            continue;
        }

        if c_reg.is_none() {
            c_reg = Some(
                line.split(": ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[1]
                    .parse::<u64>()
                    .unwrap(),
            );
            continue;
        }

        for instruction in line
            .split(": ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()[1]
            .split(",")
            .map(|s| s.to_string())
        {
            instructions.push(instruction.parse::<u64>().unwrap());
        }
    }

    Ok((a_reg.unwrap(), b_reg.unwrap(), c_reg.unwrap(), instructions))
}
