use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

const THRESHOLD: usize = 100;

pub fn run() {
    let grid = read_file();
    match grid {
        Ok(grid) => {
            let start_pos = find_start_pos(&grid).unwrap();
            part1(&grid, start_pos);
            part2(grid, start_pos);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn bfs(
    grid: &Vec<Vec<char>>,
    visited: &mut HashMap<(usize, usize), usize>,
    queue: &mut Vec<((usize, usize), usize)>,
) -> Option<usize> {
    let (height, width) = (grid.len(), grid[0].len());
    let mut index = 0;

    while index < queue.len() {
        let ((i, j), steps) = queue[index];

        if grid[i][j] == 'E' {
            return Some(steps);
        }

        if i > 0 && grid[i - 1][j] != '#' && !visited.contains_key(&(i - 1, j)) {
            queue.push(((i - 1, j), steps + 1));
            visited.insert((i - 1, j), steps + 1);
        }

        if i < height - 1 && grid[i + 1][j] != '#' && !visited.contains_key(&(i + 1, j)) {
            queue.push(((i + 1, j), steps + 1));
            visited.insert((i + 1, j), steps + 1);
        }

        if j > 0 && grid[i][j - 1] != '#' && !visited.contains_key(&(i, j - 1)) {
            queue.push(((i, j - 1), steps + 1));
            visited.insert((i, j - 1), steps + 1);
        }

        if j < width - 1 && grid[i][j + 1] != '#' && !visited.contains_key(&(i, j + 1)) {
            queue.push(((i, j + 1), steps + 1));
            visited.insert((i, j + 1), steps + 1);
        }

        index += 1;
    }
    None
}

fn find_start_pos(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_cheats(grid: &Vec<Vec<char>>, start_pos: (usize, usize), max_num_of_cheats: i8) -> usize {
    let mut queue: Vec<((usize, usize), usize)> = vec![(start_pos, 0)];
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    visited.insert(start_pos, 0);

    bfs(&grid, &mut visited, &mut queue).unwrap();

    let mut diffs: Vec<(i8, i8)> = Vec::new();

    let mut diff_i = -max_num_of_cheats;
    while diff_i < max_num_of_cheats + 1 {
        let mut diff_j = -max_num_of_cheats;
        while diff_j < max_num_of_cheats + 1 {
            if diff_i == 0 && diff_j == 0 {
                diff_j += 1;
                continue;
            }
            if diff_i.abs() + diff_j.abs() <= max_num_of_cheats {
                diffs.push((diff_j, diff_i));
            }
            diff_j += 1;
        }
        diff_i += 1;
    }

    let mut num_of_cheats = 0;
    for node in queue.iter() {
        let ((node_i, node_j), node_steps) = (node.0, node.1);
        for diff in diffs.iter() {
            let (diff_i, diff_j) = *diff;
            let coords_after_cheat = (
                (node_i as isize + diff_i as isize) as usize,
                (node_j as isize + diff_j as isize) as usize,
            );
            let cheat_len = (diff.0.abs() + diff.1.abs()) as usize;
            if visited.contains_key(&coords_after_cheat) {
                let steps_of_cheat_destination = *visited.get(&coords_after_cheat).unwrap();
                if steps_of_cheat_destination > node_steps
                    && node_steps + cheat_len + THRESHOLD <= steps_of_cheat_destination
                {
                    num_of_cheats += 1;
                }
            }
        }
    }

    num_of_cheats
}

fn part1(grid: &Vec<Vec<char>>, start_pos: (usize, usize)) {
    let num_of_cheats = find_cheats(&grid, start_pos, 2);
    println!("Part 1 Answer: {}", num_of_cheats);
}

fn part2(grid: Vec<Vec<char>>, start_pos: (usize, usize)) {
    let num_of_cheats = find_cheats(&grid, start_pos, 20);
    println!("Part 2 Answer: {}", num_of_cheats);
}

fn read_file() -> Result<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_20.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect::<Vec<char>>());
    }

    Ok(grid)
}
