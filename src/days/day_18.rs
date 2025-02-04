use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Result},
};

const WIDTH: usize = 71;
const NUM_CORRUPTIONS: usize = 1024;

pub fn run() {
    let corruptions = read_file();
    match corruptions {
        Ok(corruptions) => {
            part1(&corruptions);
            part2(&corruptions);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn bfs(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    queue: &mut VecDeque<((usize, usize), usize)>,
) -> Option<usize> {
    let mut index = 0;
    while index < queue.len() {
        let ((i, j), steps) = queue[index];

        if i == WIDTH - 1 && j == WIDTH - 1 {
            return Some(steps);
        }

        if i > 0 && grid[i - 1][j] != '#' && !visited.contains(&(i - 1, j)) {
            queue.push_back(((i - 1, j), steps + 1));
            visited.insert((i - 1, j));
        }

        if i < WIDTH - 1 && grid[i + 1][j] != '#' && !visited.contains(&(i + 1, j)) {
            queue.push_back(((i + 1, j), steps + 1));
            visited.insert((i + 1, j));
        }

        if j > 0 && grid[i][j - 1] != '#' && !visited.contains(&(i, j - 1)) {
            queue.push_back(((i, j - 1), steps + 1));
            visited.insert((i, j - 1));
        }

        if j < WIDTH - 1 && grid[i][j + 1] != '#' && !visited.contains(&(i, j + 1)) {
            queue.push_back(((i, j + 1), steps + 1));
            visited.insert((i, j + 1));
        }

        index += 1;
    }
    None
}

fn part1(corruptions: &Vec<(usize, usize)>) {
    let mut grid = vec![vec!['.'; WIDTH]; WIDTH];
    for i in 0..NUM_CORRUPTIONS {
        grid[corruptions[i].0][corruptions[i].1] = '#';
    }

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));

    let steps = bfs(&grid, &mut visited, &mut queue);
    println!("Part 1 Answer: {}", steps.unwrap());
}

fn part2(corruptions: &Vec<(usize, usize)>) {
    let mut grid = vec![vec!['.'; WIDTH]; WIDTH];
    for corruption in corruptions.iter() {
        grid[corruption.0][corruption.1] = '#';
    }

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));

    let steps = bfs(&grid, &mut visited, &mut queue);
    assert_eq!(steps, None);

    let mut corruption_to_remove = corruptions.len() - 1;
    loop {
        let corruption = corruptions[corruption_to_remove];
        grid[corruption.0][corruption.1] = '.';

        if bfs(&grid, &mut visited, &mut queue) != None {
            break;
        }

        corruption_to_remove -= 1;

        if corruption_to_remove == 0 {
            break;
        }
    }

    let cutting_corruption = corruptions[corruption_to_remove];
    println!(
        "Part 2 Answer: {},{}",
        cutting_corruption.0, cutting_corruption.1
    );
}

fn read_file() -> Result<Vec<(usize, usize)>> {
    let mut corruptions: Vec<(usize, usize)> = Vec::new();

    let file = File::open(format!("{}/inputs/day_18.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let corruption = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        corruptions.push((corruption[0], corruption[1]));
    }

    Ok(corruptions)
}
