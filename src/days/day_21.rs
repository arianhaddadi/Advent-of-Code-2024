use std::{
    char,
    cmp::min,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
    usize,
};

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A'],
];

const DIRECTIONAL_KEYBOARD: [[char; 3]; 2] = [['\0', '^', 'A'], ['<', 'v', '>']];

fn get_key_coords(keyboard_mode: KeyboardMode) -> HashMap<char, (usize, usize)> {
    let mut key_coords = HashMap::new();

    if keyboard_mode == KeyboardMode::NUMERICAL {
        for i in 0..NUMERIC_KEYPAD.len() {
            for j in 0..NUMERIC_KEYPAD[0].len() {
                key_coords.insert(NUMERIC_KEYPAD[i][j], (i, j));
            }
        }
    } else {
        for i in 0..DIRECTIONAL_KEYBOARD.len() {
            for j in 0..DIRECTIONAL_KEYBOARD[0].len() {
                key_coords.insert(DIRECTIONAL_KEYBOARD[i][j], (i, j));
            }
        }
    }

    key_coords
}

pub fn run() {
    let codes = read_file();
    match codes {
        Ok(codes) => {
            part1(&codes);
            part2(&codes);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum KeyboardMode {
    NUMERICAL,
    DIRECTIONAL,
}

fn get_possible_steps(
    src: (usize, usize),
    dst: (usize, usize),
    keyboard_mode: KeyboardMode,
) -> Vec<Vec<char>> {
    let ud_steps;
    if dst.0 > src.0 {
        ud_steps = vec!['v'; src.0.abs_diff(dst.0)];
    } else {
        ud_steps = vec!['^'; src.0.abs_diff(dst.0)];
    }

    let rl_steps;
    if dst.1 > src.1 {
        rl_steps = vec!['>'; src.1.abs_diff(dst.1)];
    } else {
        rl_steps = vec!['<'; src.1.abs_diff(dst.1)];
    }

    let mut ud_first_steps = ud_steps.clone();
    ud_first_steps.extend(rl_steps.clone());
    ud_first_steps.push('A');

    let mut rl_first_steps = rl_steps;
    rl_first_steps.extend(ud_steps);
    rl_first_steps.push('A');

    let possible_steps;
    if keyboard_mode == KeyboardMode::NUMERICAL {
        if src.1 == 0 && dst.0 == NUMERIC_KEYPAD.len() - 1 {
            possible_steps = vec![rl_first_steps];
        } else if src.0 == NUMERIC_KEYPAD.len() - 1 && dst.1 == 0 {
            possible_steps = vec![ud_first_steps];
        } else {
            possible_steps = vec![ud_first_steps, rl_first_steps];
        }
    } else {
        if src.1 == 0 && dst.0 == 0 {
            possible_steps = vec![rl_first_steps];
        } else if src.0 == 0 && dst.1 == 0 {
            possible_steps = vec![ud_first_steps];
        } else {
            possible_steps = vec![ud_first_steps, rl_first_steps];
        }
    }

    possible_steps
}

fn calculate_complexity(codes: &Vec<Vec<char>>, num_steps: &Vec<usize>) -> usize {
    let mut complexity = 0;

    for i in 0..codes.len() {
        complexity += num_steps[i]
            * codes[i].iter().collect::<String>()[0..codes[i].len() - 1]
                .parse::<usize>()
                .unwrap();
    }

    complexity
}

fn get_min_steps_length(
    code: &Vec<char>,
    num_of_keypads: u8,
    cur_keypad_num: u8,
    cache: &mut HashMap<(String, u8, KeyboardMode), usize>,
) -> usize {
    if cur_keypad_num == num_of_keypads - 1 {
        return code.len();
    }

    let code_string_form = code.iter().collect::<String>();
    let keyboard_mode = if cur_keypad_num == 0 {
        KeyboardMode::NUMERICAL
    } else {
        KeyboardMode::DIRECTIONAL
    };

    if cache.contains_key(&(code_string_form.clone(), cur_keypad_num, keyboard_mode)) {
        return *cache
            .get(&(code_string_form, cur_keypad_num, keyboard_mode))
            .unwrap();
    }

    let keys = get_key_coords(keyboard_mode);
    let mut cur_coords = *keys.get(&'A').unwrap();

    let mut min_steps_length = 0;
    for c in code.iter() {
        let dst_coords = *keys.get(c).unwrap();
        let possible_steps = get_possible_steps(cur_coords, dst_coords, keyboard_mode);

        let mut min_steps_this_step = usize::MAX;
        for steps in possible_steps.iter() {
            let length = get_min_steps_length(steps, num_of_keypads, cur_keypad_num + 1, cache);
            min_steps_this_step = min(min_steps_this_step, length);
        }
        min_steps_length += min_steps_this_step;

        cur_coords = dst_coords;
    }

    cache.insert(
        (code_string_form, cur_keypad_num, keyboard_mode),
        min_steps_length,
    );
    min_steps_length
}

fn part1(codes: &Vec<Vec<char>>) {
    let mut num_steps = Vec::new();

    let mut cache = HashMap::new();
    for code in codes.iter() {
        num_steps.push(get_min_steps_length(code, 4, 0, &mut cache));
    }

    println!("Part 1 Answer: {}", calculate_complexity(codes, &num_steps));
}

fn part2(codes: &Vec<Vec<char>>) {
    let mut num_steps = Vec::new();

    let mut cache = HashMap::new();
    for code in codes.iter() {
        num_steps.push(get_min_steps_length(code, 27, 0, &mut cache));
    }

    println!("Part 2 Answer: {}", calculate_complexity(codes, &num_steps));
}

fn read_file() -> Result<Vec<Vec<char>>> {
    let mut codes: Vec<Vec<char>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_21.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        codes.push(line.chars().collect::<Vec<char>>());
    }

    Ok(codes)
}
