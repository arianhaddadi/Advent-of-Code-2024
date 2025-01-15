use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

const GLOBAL_WIDTH: i32 = 101;
const GLOBAL_HEIGHT: i32 = 103;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

pub fn run() {
    let robots = read_file();
    match robots {
        Ok(mut robots) => {
            part1(&robots);
            part2(&mut robots);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_quadrandt_num(robot_y: i32, robot_x: i32) -> Option<usize> {
    let (mid_x, mid_y) = (GLOBAL_WIDTH / 2, GLOBAL_HEIGHT / 2);
    if robot_y == mid_y || robot_x == mid_x {
        return None;
    }
    if robot_x > mid_x {
        if robot_y > mid_y {
            return Some(3);
        }
        return Some(1);
    }
    if robot_y > mid_y {
        return Some(2);
    }
    Some(0)
}

fn move_1_sec(robot_y: i32, robot_x: i32, y_diff: i32, x_diff: i32) -> (i32, i32) {
    let mut new_y = robot_y + y_diff;
    if new_y < 0 {
        new_y = GLOBAL_HEIGHT + new_y;
    }
    new_y = new_y % GLOBAL_HEIGHT;

    let mut new_x = robot_x + x_diff;
    if new_x < 0 {
        new_x = GLOBAL_WIDTH + new_x;
    }
    new_x = new_x % GLOBAL_WIDTH;

    (new_x, new_y)
}

fn move_100_secs(robot: Robot) -> Option<usize> {
    let y_diff = ((100 * robot.vy.abs()) % GLOBAL_HEIGHT) * (robot.vy / robot.vy.abs());
    let x_diff = ((100 * robot.vx.abs()) % GLOBAL_WIDTH) * (robot.vx / robot.vx.abs());
    let (new_x, new_y) = move_1_sec(robot.y, robot.x, y_diff, x_diff);

    get_quadrandt_num(new_y, new_x)
}

fn part1(robots: &Vec<Robot>) {
    let mut num_of_robots = vec![0, 0, 0, 0];
    for robot in robots.iter() {
        let quadrant = move_100_secs(*robot);
        if quadrant == None {
            continue;
        }
        num_of_robots[quadrant.unwrap()] += 1;
    }

    let safety_factor = num_of_robots.iter().fold(1, |acc, &x| acc * x);
    println!("Part 1 Answer: {}", safety_factor);
}

fn all_locs_are_unique(robots_vec: &Vec<Robot>) -> bool {
    let mut robots_set: HashSet<(i32, i32)> = HashSet::new();
    for robot in robots_vec.iter() {
        robots_set.insert((robot.x, robot.y));
    }
    robots_set.len() == robots_vec.len()
}

fn part2(robots: &mut Vec<Robot>) {
    let mut seconds = 0;
    loop {
        for robot in robots.iter_mut() {
            (robot.x, robot.y) = move_1_sec(robot.y, robot.x, robot.vy, robot.vx)
        }
        seconds += 1;
        if all_locs_are_unique(&robots) {
            println!("Part 2 Answer: {}", seconds);
            break;
        }
    }
}

fn read_file() -> Result<Vec<Robot>> {
    let mut robots: Vec<Robot> = Vec::new();

    let file = File::open(format!("{}/inputs/day_14.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let info = line.split_whitespace().collect::<Vec<&str>>();
        let positions = info[0][2..]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let velocities = info[1][2..]
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        robots.push(Robot {
            x: positions[0],
            y: positions[1],
            vx: velocities[0],
            vy: velocities[1],
        })
    }

    Ok(robots)
}
