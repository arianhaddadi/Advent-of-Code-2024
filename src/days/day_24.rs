use std::{
    collections::{HashMap, HashSet},
    fmt,
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::AND => {
                write!(f, "AND")
            }
            Self::OR => {
                write!(f, "OR")
            }
            Self::XOR => {
                write!(f, "XOR")
            }
        }
    }
}

impl Operation {
    fn get_operation(repr: String) -> Self {
        if repr == "XOR" {
            return Self::XOR;
        }
        if repr == "OR" {
            return Self::OR;
        }
        Self::AND
    }

    fn calculate_result(operation: Self, val_1: bool, val_2: bool) -> bool {
        match operation {
            Self::AND => val_1 && val_2,
            Self::OR => val_1 || val_2,
            Self::XOR => val_1 ^ val_2,
        }
    }
}

struct Gate {
    input_1: String,
    input_2: String,
    result: String,
    operation: Operation,
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Gate(input_1: {}, input_2: {}, operation: {}, result: {})",
            self.input_1, self.input_2, self.operation, self.result
        )
    }
}

pub fn run() {
    let file_content = read_file();
    match file_content {
        Ok((values, gates)) => {
            part1(values, &gates);
            part2(gates);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_decimal_form(values: &HashMap<String, Option<bool>>, input: char) -> u64 {
    let mut i = 0;
    let mut result = String::new();
    loop {
        let variable_name = input.to_string() + format!("{:02}", i).as_str();

        if !values.contains_key(&variable_name) {
            break;
        }

        if values.get(&variable_name).unwrap().unwrap() {
            result.push('1');
        } else {
            result.push('0');
        }
        i += 1;
    }

    result = result.chars().rev().collect::<String>();
    u64::from_str_radix(result.as_str(), 2).unwrap()
}

fn calculate_values(values: &mut HashMap<String, Option<bool>>, gates: &Vec<Gate>) {
    let mut solved: HashSet<usize> = HashSet::new();
    while solved.len() < gates.len() {
        for i in (0..gates.len()).rev() {
            if solved.contains(&i) {
                continue;
            }
            let gate = &gates[i];
            if *values.get(&gate.input_1).unwrap() != None
                && *values.get(&gate.input_2).unwrap() != None
            {
                let val_1 = values.get(&gate.input_1).unwrap().unwrap();
                let val_2 = values.get(&gate.input_2).unwrap().unwrap();
                values.insert(
                    gate.result.clone(),
                    Some(Operation::calculate_result(gate.operation, val_1, val_2)),
                );
                solved.insert(i);
            }
        }
    }
}

fn get_output_length(gates: &Vec<Gate>) -> usize {
    let mut max_z_index = 0;
    for gate in gates.iter() {
        if gate.result.chars().nth(0).unwrap() == 'z' {
            let index = gate.result.as_str()[1..].parse::<usize>().unwrap();
            if index > max_z_index {
                max_z_index = index;
            }
        }
    }
    max_z_index
}

fn part1(mut values: HashMap<String, Option<bool>>, gates: &Vec<Gate>) {
    calculate_values(&mut values, gates);
    println!("Part 1 Answer: {}", get_decimal_form(&values, 'z'));
}

fn part2(gates: Vec<Gate>) {
    let mut wrong_results: HashSet<String> = HashSet::new();

    let output_length = get_output_length(&gates);

    for gate in gates.iter() {
        if gate.result.chars().nth(0).unwrap() == 'z'
            && gate.operation != Operation::XOR
            && gate.result != 'z'.to_string() + output_length.to_string().as_str()
        {
            wrong_results.insert(gate.result.clone());
        }

        if gate.operation == Operation::OR && gate.result.chars().nth(0).unwrap() != 'z' {
            for inner_loop_gate in gates.iter() {
                if (inner_loop_gate.input_1 == gate.result
                    || inner_loop_gate.input_2 == gate.result)
                    && inner_loop_gate.operation == Operation::OR
                {
                    wrong_results.insert(inner_loop_gate.result.clone());
                    break;
                }
            }
        }

        if gate.operation == Operation::OR {
            for inner_loop_gate in gates.iter() {
                if (inner_loop_gate.result == gate.input_1
                    || inner_loop_gate.result == gate.input_2)
                    && inner_loop_gate.operation != Operation::AND
                {
                    wrong_results.insert(inner_loop_gate.result.clone());
                }
            }
        }

        if gate.operation == Operation::AND && gate.input_1 != "x00" && gate.input_2 != "x00" {
            for inner_loop_gate in gates.iter() {
                if (inner_loop_gate.input_1 == gate.result
                    || inner_loop_gate.input_2 == gate.result)
                    && inner_loop_gate.operation != Operation::OR
                {
                    wrong_results.insert(gate.result.clone());
                    break;
                }
            }
        }

        if gate.operation == Operation::XOR
            && (gate.input_1.chars().nth(0).unwrap() != 'x'
                && gate.input_1.chars().nth(0).unwrap() != 'y')
        {
            for inner_loop_gate in gates.iter() {
                if inner_loop_gate.input_1 == gate.result || inner_loop_gate.input_2 == gate.result
                {
                    wrong_results.insert(gate.result.clone());
                    break;
                }
            }
        }
    }

    let mut wrong_results = wrong_results.into_iter().collect::<Vec<String>>();
    wrong_results.sort();
    println!("Part 2 Answer: {}", wrong_results.join(","));
}

fn read_file() -> Result<(HashMap<String, Option<bool>>, Vec<Gate>)> {
    let mut values: HashMap<String, Option<bool>> = HashMap::new();
    let mut gates: Vec<Gate> = Vec::new();

    let file = File::open(format!("{}/inputs/day_24.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    let mut is_values_phase = true;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            is_values_phase = false;
            continue;
        }

        if is_values_phase {
            let mut content = line
                .split(':')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            let value = content.remove(1);
            let variable = content.remove(0);
            values.insert(variable, Some(value == "1"));
            continue;
        }

        let mut content = line
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        let result = content.remove(4);
        let input_2 = content.remove(2);
        let operation = content.remove(1);
        let input_1 = content.remove(0);

        if !values.contains_key(&input_1) {
            values.insert(input_1.clone(), None);
        }
        if !values.contains_key(&input_2) {
            values.insert(input_2.clone(), None);
        }
        if !values.contains_key(&result) {
            values.insert(result.clone(), None);
        }

        gates.push(Gate {
            input_1,
            input_2,
            result,
            operation: Operation::get_operation(operation),
        });
    }

    Ok((values, gates))
}
