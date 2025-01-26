use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let file_content = read_file();
    match file_content {
        Ok((moves, grid)) => {
            part1(&moves, grid.clone());
            part2(&moves, grid);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_initial_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn can_move_up(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[i - 1][j] == '#' {
        return false;
    }

    if grid[i - 1][j] == '.' {
        return true;
    }

    if grid[i - 1][j] == 'O' {
        return can_move_up(i - 1, j, grid);
    }

    if grid[i - 1][j] == '[' {
        return can_move_up(i - 1, j, grid) && can_move_up(i - 1, j + 1, grid);
    }

    // grid[i - 1][j] == ']'
    can_move_up(i - 1, j, grid) && can_move_up(i - 1, j - 1, grid)
}

fn move_up(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    if grid[i - 1][j] == '[' {
        move_up(i - 1, j, grid);
        move_up(i - 1, j + 1, grid);
    }

    if grid[i - 1][j] == ']' {
        move_up(i - 1, j, grid);
        move_up(i - 1, j - 1, grid);
    }

    if grid[i - 1][j] == 'O' {
        move_up(i - 1, j, grid);
    }

    grid[i - 1][j] = grid[i][j];
    grid[i][j] = '.';
}

fn check_and_move_up(position: &mut (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (i, j) = *position;
    if grid[i - 1][j] == '#' {
        return;
    }

    if grid[i - 1][j] == 'O' || grid[i - 1][j] == '[' || grid[i - 1][j] == ']' {
        if !can_move_up(i, j, grid) {
            return;
        }
        move_up(i, j, grid);
    }

    grid[i][j] = '.';
    grid[i - 1][j] = '@';
    position.0 -= 1;
}

fn can_move_down(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[i + 1][j] == '#' {
        return false;
    }

    if grid[i + 1][j] == '.' {
        return true;
    }

    if grid[i + 1][j] == 'O' {
        return can_move_down(i + 1, j, grid);
    }

    if grid[i + 1][j] == '[' {
        return can_move_down(i + 1, j, grid) && can_move_down(i + 1, j + 1, grid);
    }

    // grid[i + 1][j] == ']'
    can_move_down(i + 1, j, grid) && can_move_down(i + 1, j - 1, grid)
}

fn move_down(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
    if grid[i + 1][j] == '[' {
        move_down(i + 1, j, grid);
        move_down(i + 1, j + 1, grid);
    }

    if grid[i + 1][j] == ']' {
        move_down(i + 1, j, grid);
        move_down(i + 1, j - 1, grid);
    }

    if grid[i + 1][j] == 'O' {
        move_down(i + 1, j, grid);
    }

    grid[i + 1][j] = grid[i][j];
    grid[i][j] = '.';
}

fn check_and_move_down(position: &mut (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (i, j) = *position;
    if grid[i + 1][j] == '#' {
        return;
    }

    if grid[i + 1][j] == 'O' || grid[i + 1][j] == '[' || grid[i + 1][j] == ']' {
        if !can_move_down(i, j, grid) {
            return;
        }
        move_down(i, j, grid);
    }

    grid[i][j] = '.';
    grid[i + 1][j] = '@';
    position.0 += 1;
}

fn check_and_move_right(position: &mut (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (i, j) = *position;
    if grid[i][j + 1] == '#' {
        return;
    }

    if grid[i][j + 1] == 'O' || grid[i][j + 1] == '[' {
        let mut blocks_j = j + 1;
        while grid[i][blocks_j] == 'O' || grid[i][blocks_j] == '[' || grid[i][blocks_j] == ']' {
            blocks_j += 1;
        }

        if grid[i][blocks_j] == '#' {
            return;
        }
        while blocks_j > j + 1 {
            grid[i][blocks_j] = grid[i][blocks_j - 1];
            blocks_j -= 1;
        }
    }

    grid[i][j] = '.';
    grid[i][j + 1] = '@';
    position.1 += 1;
}

fn check_and_move_left(position: &mut (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (i, j) = *position;
    if grid[i][j - 1] == '#' {
        return;
    }

    if grid[i][j - 1] == 'O' || grid[i][j - 1] == ']' {
        let mut blocks_j = j - 1;
        while grid[i][blocks_j] == 'O' || grid[i][blocks_j] == '[' || grid[i][blocks_j] == ']' {
            blocks_j -= 1;
        }

        if grid[i][blocks_j] == '#' {
            return;
        }
        while blocks_j < j - 1 {
            grid[i][blocks_j] = grid[i][blocks_j + 1];
            blocks_j += 1;
        }
    }

    grid[i][j] = '.';
    grid[i][j - 1] = '@';
    position.1 -= 1;
}

fn get_gps_sum(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' || grid[i][j] == '[' {
                sum += (i * 100) + j;
            }
        }
    }
    sum
}

fn part1(moves: &Vec<char>, mut grid: Vec<Vec<char>>) {
    let mut initial_position = get_initial_position(&grid);

    for movement in moves.iter() {
        match movement {
            '^' => {
                check_and_move_up(&mut initial_position, &mut grid);
            }
            'v' => {
                check_and_move_down(&mut initial_position, &mut grid);
            }
            '<' => {
                check_and_move_left(&mut initial_position, &mut grid);
            }
            '>' => {
                check_and_move_right(&mut initial_position, &mut grid);
            }
            _ => {}
        }
    }

    let gps_sum = get_gps_sum(&mut grid);

    println!("Part 1 Answer: {}", gps_sum);
}

fn scale_up(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut scaled_grid: Vec<Vec<char>> = Vec::new();

    for i in 0..grid.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                row.push('#');
                row.push('#');
            } else if grid[i][j] == '@' {
                row.push('@');
                row.push('.');
            } else if grid[i][j] == 'O' {
                row.push('[');
                row.push(']');
            } else {
                row.push('.');
                row.push('.');
            }
        }
        scaled_grid.push(row);
    }

    scaled_grid
}

fn part2(moves: &Vec<char>, grid: Vec<Vec<char>>) {
    let mut grid = scale_up(&grid);
    let mut initial_position = get_initial_position(&grid);

    for movement in moves.iter() {
        match movement {
            '^' => {
                check_and_move_up(&mut initial_position, &mut grid);
            }
            'v' => {
                check_and_move_down(&mut initial_position, &mut grid);
            }
            '<' => {
                check_and_move_left(&mut initial_position, &mut grid);
            }
            '>' => {
                check_and_move_right(&mut initial_position, &mut grid);
            }
            _ => {}
        }
    }

    let gps_sum = get_gps_sum(&mut grid);

    println!("Part 2 Answer: {}", gps_sum);
}

fn read_file() -> Result<(Vec<char>, Vec<Vec<char>>)> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    let file = File::open(format!("{}/inputs/day_15.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    let mut is_grid = true;
    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            is_grid = false;
            continue;
        }

        if is_grid {
            grid.push(line.chars().collect());
        } else {
            moves.extend(line.chars().collect::<Vec<char>>());
        }
    }

    Ok((moves, grid))
}
