use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let connections = read_file();
    match connections {
        Ok(connections) => {
            part1(&connections);
            part2(&connections);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn part1(connections: &HashMap<String, (Vec<String>, HashSet<String>)>) {
    let mut groups: HashSet<String> = HashSet::new();
    for node in connections.keys() {
        if node.chars().nth(0).unwrap() == 't' {
            let group = &connections.get(node).unwrap().0;
            for i in 0..group.len() - 1 {
                for j in i..group.len() {
                    if !connections.get(&group[i]).unwrap().1.contains(&group[j])
                        || !connections.get(&group[j]).unwrap().1.contains(&group[i])
                    {
                        continue;
                    }
                    let mut group = vec![node.clone(), group[i].clone(), group[j].clone()];
                    group.sort();
                    groups.insert(group.join("_"));
                }
            }
        }
    }
    println!("Part 1 Answer: {}", groups.len());
}

fn forms_party(
    group: &Vec<&String>,
    candidate: &String,
    connections: &HashMap<String, (Vec<String>, HashSet<String>)>,
) -> bool {
    let candidate_connections = &connections.get(candidate).unwrap().1;
    for node in group {
        if !candidate_connections.contains(*node)
            || !connections.get(*node).unwrap().1.contains(candidate)
        {
            return false;
        }
    }
    true
}

fn backtrack<'a: 'b, 'b>(
    neighbours: &'a Vec<String>,
    connections: &HashMap<String, (Vec<String>, HashSet<String>)>,
    password: &mut String,
    group: &mut Vec<&'b String>,
    index: usize,
) {
    for i in index..neighbours.len() {
        if forms_party(group, &neighbours[i], connections) {
            group.push(&neighbours[i]);
            if group.len() > password.split("_").collect::<Vec<&str>>().len() {
                *password = group
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>()
                    .join("_");
            }
            backtrack(neighbours, connections, password, group, i + 1);
            group.pop();
        }
    }
}

fn part2(connections: &HashMap<String, (Vec<String>, HashSet<String>)>) {
    let mut password = String::new();
    let mut group: Vec<&String> = Vec::new();

    for node in connections.keys() {
        group.push(node);
        backtrack(
            &connections.get(node).unwrap().0,
            connections,
            &mut password,
            &mut group,
            1,
        );
        group.clear();
    }

    let mut password_nodes = password
        .split("_")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    password_nodes.sort();
    password = password_nodes.join(",");

    println!("Part 2 Answer: {}", password);
}

fn read_file() -> Result<HashMap<String, (Vec<String>, HashSet<String>)>> {
    let mut connections: HashMap<String, (Vec<String>, HashSet<String>)> = HashMap::new();

    let file = File::open(format!("{}/inputs/day_23.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let nodes = line.split("-").collect::<Vec<&str>>();
        connections
            .entry(nodes[0].to_string())
            .or_insert_with(|| (Vec::new(), HashSet::new()));
        connections
            .get_mut(&nodes[0].to_string())
            .unwrap()
            .0
            .push(nodes[1].to_string());
        connections
            .get_mut(&nodes[0].to_string())
            .unwrap()
            .1
            .insert(nodes[1].to_string());

        connections
            .entry(nodes[1].to_string())
            .or_insert_with(|| (Vec::new(), HashSet::new()));
        connections
            .get_mut(&nodes[1].to_string())
            .unwrap()
            .0
            .push(nodes[0].to_string());
        connections
            .get_mut(&nodes[1].to_string())
            .unwrap()
            .1
            .insert(nodes[0].to_string());
    }

    Ok(connections)
}
