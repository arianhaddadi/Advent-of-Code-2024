use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let grid = read_file();
    match grid {
        Ok(grid) => {
            part1(&grid);
            part2(&grid);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn bfs(grid: &Vec<Vec<char>>) -> (u64, usize) {
    let mut queue: VecDeque<((usize, usize), Direction, u64, HashSet<(usize, usize)>)> =
        VecDeque::new();
    queue.push_back((
        (grid.len() - 2, 1),
        Direction::RIGHT,
        0,
        HashSet::from([(grid.len() - 2, 1)]),
    ));

    let mut passed_coords: HashMap<(usize, usize), u64> = HashMap::new();

    let mut best_spots: HashSet<(usize, usize)> = HashSet::new();

    let (height, width) = (grid.len(), grid[0].len());

    let mut min_cost: Option<u64> = None;

    while !queue.is_empty() {
        let ((i, j), direction, cost, path) = queue.pop_back().unwrap();

        if i == 1 && j == width - 2 {
            //  We have reached the end
            if min_cost == None || min_cost.unwrap() >= cost {
                if min_cost == None || cost < min_cost.unwrap() {
                    best_spots.clear();
                }
                min_cost = Some(cost);
                for coord in path.iter() {
                    best_spots.insert(*coord);
                }
            }
            continue;
        }

        if passed_coords.contains_key(&(i, j)) && *passed_coords.get(&(i, j)).unwrap() < cost - 1000
        {
            continue;
        }
        passed_coords.insert((i, j), cost);

        // One Step Down
        if i < height - 2 && grid[i + 1][j] != '#' && direction != Direction::UP {
            let mut new_cost = cost + 1;
            if direction != Direction::DOWN {
                new_cost += 1000;
            }
            let new_coords = (i + 1, j);
            let mut path_clone = path.clone();
            path_clone.insert(new_coords);
            queue.push_back((new_coords, Direction::DOWN, new_cost, path_clone));
        }

        // One Step Up
        if i > 1 && grid[i - 1][j] != '#' && direction != Direction::DOWN {
            let mut new_cost = cost + 1;
            if direction != Direction::UP {
                new_cost += 1000;
            }
            let new_coords = (i - 1, j);
            let mut path_clone = path.clone();
            path_clone.insert(new_coords);
            queue.push_back((new_coords, Direction::UP, new_cost, path_clone));
        }

        // One Step Right
        if j < width - 2 && grid[i][j + 1] != '#' && direction != Direction::LEFT {
            let mut new_cost = cost + 1;
            if direction != Direction::RIGHT {
                new_cost += 1000;
            }
            let new_coords = (i, j + 1);
            let mut path_clone = path.clone();
            path_clone.insert(new_coords);
            queue.push_back((new_coords, Direction::RIGHT, new_cost, path_clone));
        }

        // One Step Left
        if j > 1 && grid[i][j - 1] != '#' && direction != Direction::RIGHT {
            let mut new_cost = cost + 1;
            if direction != Direction::LEFT {
                new_cost += 1000;
            }
            let new_coords = (i, j - 1);
            let mut path_clone = path.clone();
            path_clone.insert(new_coords);
            queue.push_back((new_coords, Direction::LEFT, new_cost, path_clone));
        }
    }

    if min_cost == None {
        return (0, 0);
    }
    (min_cost.unwrap(), best_spots.len())
}

fn part1(grid: &Vec<Vec<char>>) {
    println!("Part 1 Answer: {}", bfs(grid).0);
}

fn part2(grid: &Vec<Vec<char>>) {
    println!("Part 2 Answer: {}", bfs(grid).1);
}

fn read_file() -> Result<Vec<Vec<char>>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let file = File::open(format!("{}/inputs/day_16.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let digits: Vec<char> = line.chars().collect();
        grid.push(digits);
    }

    // Make sure start and end are at the expected coordinations
    assert!(grid[grid.len() - 2][1] == 'S' && grid[1][grid[0].len() - 2] == 'E');

    Ok(grid)
}
