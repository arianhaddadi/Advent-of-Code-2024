use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn get_next_direction(cur_dir: &Direction) -> Self {
        match *cur_dir {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn rotate(&mut self) {
        *self = Direction::get_next_direction(self)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::UP => write!(f, "UP"),
            Direction::DOWN => write!(f, "DOWN"),
            Direction::LEFT => write!(f, "LEFT"),
            Direction::RIGHT => write!(f, "RIGHT"),
        }
    }
}

pub fn run() {
    let map = read_file();
    match map {
        Ok(map) => {
            let initial_location = find_initial_loc(&map);
            part1(&map, initial_location);
            part2(&map, initial_location);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn find_initial_loc(map: &Vec<String>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i].chars().nth(j).unwrap();
            if c == '^' {
                return (i, j);
            }
        }
    }

    (0, 0)
}

fn get_next_coord(current_coord: (usize, usize), direction: Direction) -> (usize, usize) {
    let (mut i, mut j) = current_coord;
    if direction == Direction::UP {
        i -= 1;
    }
    if direction == Direction::DOWN {
        i += 1;
    }
    if direction == Direction::LEFT {
        j -= 1;
    }
    if direction == Direction::RIGHT {
        j += 1;
    }

    (i, j)
}

fn is_next_coord_out(
    current_coord: (usize, usize),
    direction: Direction,
    map: &Vec<String>,
) -> bool {
    let (i, j) = current_coord;

    (direction == Direction::UP && i == 0)
        || (direction == Direction::DOWN && i == map.len() - 1)
        || (direction == Direction::LEFT && j == 0)
        || (direction == Direction::RIGHT && j == map[0].len() - 1)
}

fn traverse(
    map: &Vec<String>,
    initial_location: (usize, usize),
    added_obstacles: HashSet<(usize, usize)>,
) -> (HashMap<(usize, usize), Direction>, bool) {
    let mut passed_coords: HashMap<(usize, usize), Direction> = HashMap::new();
    let mut traversed: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut has_loop = false;

    let mut direction = Direction::UP;
    let (mut i, mut j) = initial_location;
    loop {
        if traversed.contains(&(i, j, direction)) {
            has_loop = true;
            break;
        }

        passed_coords.insert((i, j), direction);
        traversed.insert((i, j, direction));

        if is_next_coord_out((i, j), direction, map) {
            break;
        }

        let (next_i, next_j) = get_next_coord((i, j), direction);
        if map[next_i].chars().nth(next_j).unwrap() == '#'
            || added_obstacles.contains(&(next_i, next_j))
        {
            direction.rotate();
            traversed.remove(&(i, j, direction));
            continue;
        }

        (i, j) = (next_i, next_j);
    }

    (passed_coords, has_loop)
}

fn part1(map: &Vec<String>, initial_location: (usize, usize)) {
    let (passed_coords, _) = traverse(map, initial_location, HashSet::new());
    println!("Part 1 Answer: {}", passed_coords.len());
}

fn part2(map: &Vec<String>, initial_location: (usize, usize)) {
    let mut obstacles_to_add: HashSet<(usize, usize)> = HashSet::new();
    let mut successful_obstacles = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i].chars().nth(j).unwrap() != '#' {
                obstacles_to_add.insert((i, j));
            }
        }
    }

    for obstacle in obstacles_to_add {
        let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
        obstacles.insert(obstacle);
        if traverse(map, initial_location, obstacles).1 == true {
            successful_obstacles += 1;
        }
    }

    println!("Part 2 Answer: {}", successful_obstacles);
}

fn read_file() -> Result<Vec<String>> {
    let mut map: Vec<String> = Vec::new();
    let file = File::open("../inputs/day_6.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        map.push(line);
    }
    Ok(map)
}
