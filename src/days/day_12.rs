use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let grid = read_file();
    match grid {
        Ok(grid) => {
            part1(grid.clone());
            part2(grid);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn traverse(
    grid: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    passed_coords: &mut HashSet<(usize, usize)>,
    sides: &mut HashSet<(usize, usize, Direction)>,
) -> (u32, u32) {
    let (mut area, mut perimeter) = (1, 0);
    let (height, width) = (grid.len(), grid[0].len());
    let character = grid[i][j];
    grid[i][j] = '.';
    passed_coords.insert((i, j));

    if i > 0 && grid[i - 1][j] == character {
        let result = traverse(grid, i - 1, j, passed_coords, sides);
        area += result.0;
        perimeter += result.1;
    } else if i == 0 || !passed_coords.contains(&(i - 1, j)) {
        sides.insert((i, j, Direction::UP));
        perimeter += 1;
    }

    if i < height - 1 && grid[i + 1][j] == character {
        let result = traverse(grid, i + 1, j, passed_coords, sides);
        area += result.0;
        perimeter += result.1;
    } else if !passed_coords.contains(&(i + 1, j)) {
        sides.insert((i, j, Direction::DOWN));
        perimeter += 1;
    }

    if j < width - 1 && grid[i][j + 1] == character {
        let result = traverse(grid, i, j + 1, passed_coords, sides);
        area += result.0;
        perimeter += result.1;
    } else if !passed_coords.contains(&(i, j + 1)) {
        sides.insert((i, j, Direction::RIGHT));
        perimeter += 1;
    }

    if j > 0 && grid[i][j - 1] == character {
        let result = traverse(grid, i, j - 1, passed_coords, sides);
        area += result.0;
        perimeter += result.1;
    } else if j == 0 || !passed_coords.contains(&(i, j - 1)) {
        sides.insert((i, j, Direction::LEFT));
        perimeter += 1;
    }

    (area, perimeter)
}

fn part1(mut grid: Vec<Vec<char>>) {
    let (height, width) = (grid.len(), grid[0].len());
    let mut total_cost = 0;
    let mut passed_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut sides: HashSet<(usize, usize, Direction)> = HashSet::new();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '.' {
                continue;
            }
            let (area, perimeter) = traverse(&mut grid, i, j, &mut passed_coords, &mut sides);
            total_cost += area * perimeter;
            passed_coords.clear();
            sides.clear();
        }
    }
    println!("Part 1 Answer: {}", total_cost);
}

fn get_num_of_sides(
    sides: &mut HashSet<(usize, usize, Direction)>,
    passed_coords: &mut HashSet<(usize, usize)>,
    height: usize,
    width: usize,
) -> u32 {
    let mut num_of_sides = 0;
    while sides.len() > 0 {
        let (x, y, direction) = *sides.iter().next().unwrap();
        sides.remove(&(x, y, direction));
        num_of_sides += 1;
        if direction == Direction::UP || direction == Direction::DOWN {
            let (mut i, mut j) = (x, y);
            while j > 0
                && passed_coords.contains(&(i, j - 1))
                && sides.contains(&(i, j - 1, direction))
            {
                j -= 1;
                sides.remove(&(i, j, direction));
            }
            (i, j) = (x, y);
            while j < width - 1
                && passed_coords.contains(&(i, j + 1))
                && sides.contains(&(i, j + 1, direction))
            {
                j += 1;
                sides.remove(&(i, j, direction));
            }
        }
        if direction == Direction::LEFT || direction == Direction::RIGHT {
            let (mut i, mut j) = (x, y);
            while i > 0
                && passed_coords.contains(&(i - 1, j))
                && sides.contains(&(i - 1, j, direction))
            {
                i -= 1;
                sides.remove(&(i, j, direction));
            }
            (i, j) = (x, y);
            while i < height - 1
                && passed_coords.contains(&(i + 1, j))
                && sides.contains(&(i + 1, j, direction))
            {
                i += 1;
                sides.remove(&(i, j, direction));
            }
        }
    }
    num_of_sides
}

fn part2(mut grid: Vec<Vec<char>>) {
    let (height, width) = (grid.len(), grid[0].len());
    let mut total_cost = 0;
    let mut passed_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut sides: HashSet<(usize, usize, Direction)> = HashSet::new();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '.' {
                continue;
            }
            let (area, _) = traverse(&mut grid, i, j, &mut passed_coords, &mut sides);
            total_cost += area * get_num_of_sides(&mut sides, &mut passed_coords, height, width);
            passed_coords.clear();
            sides.clear();
        }
    }
    println!("Part 2 Answer: {}", total_cost);
}

fn read_file() -> Result<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_12.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<char> = line.chars().collect();
        grid.push(digits);
    }

    Ok(grid)
}
