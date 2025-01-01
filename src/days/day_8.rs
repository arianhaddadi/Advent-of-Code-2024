use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let lines = read_file();
    match lines {
        Ok(map) => {
            let antenna_locations = process_map(&map);
            part1(&map, &antenna_locations);
            part2(&map, &antenna_locations);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn loc_is_within_boundaries(map: &Vec<String>, loc: (isize, isize)) -> bool {
    let (height, width) = (map.len(), map[0].len());
    if loc.0 >= 0 && loc.0 < height as isize && loc.1 >= 0 && loc.1 < width as isize {
        return true;
    }
    false
}

fn part1(map: &Vec<String>, antenna_locations: &HashMap<char, Vec<(isize, isize)>>) {
    let mut antinodes_locations: HashSet<(isize, isize)> = HashSet::new();

    for antenna in antenna_locations.keys() {
        let locs: &Vec<(isize, isize)> = antenna_locations.get(antenna).unwrap();

        for i in 0..locs.len() - 1 {
            for j in (i + 1)..locs.len() {
                let (antenna_1, antenna_2) = (locs[i], locs[j]);
                let (i_diff, j_diff) = (antenna_2.0 - antenna_1.0, antenna_2.1 - antenna_1.1);
                let new_loc_1 = (antenna_1.0 - i_diff, antenna_1.1 - j_diff);
                let new_loc_2 = (antenna_2.0 + i_diff, antenna_2.1 + j_diff);

                if loc_is_within_boundaries(map, new_loc_1) {
                    antinodes_locations.insert(new_loc_1);
                }

                if loc_is_within_boundaries(map, new_loc_2) {
                    antinodes_locations.insert(new_loc_2);
                }
            }
        }
    }

    println!("Part 1 Answer: {}", antinodes_locations.len());
}

fn part2(map: &Vec<String>, antenna_locations: &HashMap<char, Vec<(isize, isize)>>) {
    let mut antinodes_locations: HashSet<(isize, isize)> = HashSet::new();

    for antenna in antenna_locations.keys() {
        let locs: &Vec<(isize, isize)> = antenna_locations.get(antenna).unwrap();

        for i in 0..locs.len() - 1 {
            for j in (i + 1)..locs.len() {
                let (antenna_1, antenna_2) = (locs[i], locs[j]);
                let (i_diff, j_diff) = (antenna_2.0 - antenna_1.0, antenna_2.1 - antenna_1.1);
                let mut new_loc_1 = antenna_1;
                let mut new_loc_2 = antenna_2;

                while loc_is_within_boundaries(map, new_loc_1) {
                    antinodes_locations.insert(new_loc_1);
                    new_loc_1 = (new_loc_1.0 - i_diff, new_loc_1.1 - j_diff);
                }

                while loc_is_within_boundaries(map, new_loc_2) {
                    antinodes_locations.insert(new_loc_2);
                    new_loc_2 = (new_loc_2.0 + i_diff, new_loc_2.1 + j_diff);
                }
            }
        }
    }

    println!("Part 2 Answer: {}", antinodes_locations.len());
}

fn process_map(map: &Vec<String>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antenna_locations: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let c = map[i].chars().nth(j).unwrap();

            if c == '.' {
                continue;
            }

            antenna_locations
                .entry(c)
                .or_insert_with(Vec::new)
                .push((i as isize, j as isize));
        }
    }

    antenna_locations
}

fn read_file() -> Result<Vec<String>> {
    let mut map: Vec<String> = Vec::new();
    let file = File::open("../inputs/day_8.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        map.push(line);
    }

    Ok(map)
}
