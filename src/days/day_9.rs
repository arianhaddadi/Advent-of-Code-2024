use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn run() {
    let disk_map = read_file();
    match disk_map {
        Ok(disk_map) => {
            part1(&disk_map);
            part2(&disk_map);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn get_disk_content_block(disk_map: &String) -> Vec<Option<u64>> {
    let mut id: u64 = 0;
    let mut content: Vec<Option<u64>> = Vec::new();
    let mut is_blank = false;
    for c in disk_map.chars() {
        let num = c.to_digit(10).unwrap();

        let mut val: Option<u64> = None;

        if !is_blank {
            val = Some(id);
            id += 1;
        }

        for _ in 0..num {
            content.push(val);
        }

        is_blank = !is_blank;
    }
    content
}

fn compress_disk_content_block(disk_content: &mut Vec<Option<u64>>) {
    let (mut i, mut j) = (0, disk_content.len() - 1);
    while i < j {
        while i < disk_content.len() && disk_content[i] != None {
            i += 1;
        }

        while j > i && disk_content[j] == None {
            j -= 1;
        }

        if i < j {
            disk_content[i] = disk_content[j];
            disk_content[j] = None;
        }
    }
}

fn calculate_checksum(disk_content: &Vec<Option<u64>>) -> u128 {
    let mut checksum = 0;

    for i in 1..disk_content.len() {
        if disk_content[i] == None {
            continue;
        }
        checksum += (i as u128) * disk_content[i].unwrap() as u128;
    }

    checksum
}

fn part1(disk_map: &String) {
    let mut disk_content_block = get_disk_content_block(&disk_map);
    compress_disk_content_block(&mut disk_content_block);
    let checksum: u128 = calculate_checksum(&disk_content_block);
    println!("Part 1 Answer: {}", checksum);
}

fn get_disk_content_file(disk_map: &String) -> Vec<(u32, Option<u64>)> {
    let mut id: u64 = 0;
    let mut content: Vec<(u32, Option<u64>)> = Vec::new();
    let mut is_blank = false;
    for c in disk_map.chars() {
        let num = c.to_digit(10).unwrap();

        if is_blank {
            content.push((num, None));
        } else {
            content.push((num, Some(id)));
            id += 1;
        }

        is_blank = !is_blank;
    }
    content
}

fn compress_disk_content_file(disk_content: &mut Vec<(u32, Option<u64>)>) {
    let mut file_index: usize = disk_content.len() - 1;
    let mut processed_files: HashSet<u64> = HashSet::new();

    while file_index > 1 {
        if disk_content[file_index].1 == None {
            file_index -= 1;
            continue;
        }
        assert_ne!(disk_content[file_index].1, None);

        if processed_files.contains(&disk_content[file_index].1.unwrap()) {
            file_index -= 1;
            continue;
        }

        processed_files.insert(disk_content[file_index].1.unwrap());
        for i in 0..file_index {
            if disk_content[i].1 != None {
                continue;
            }
            assert_eq!(disk_content[i].1, None);

            if disk_content[i].0 >= disk_content[file_index].0 {
                disk_content[i].0 -= disk_content[file_index].0;
                disk_content.insert(i, disk_content[file_index]);
                file_index += 1;
                disk_content[file_index].1 = None;
                break;
            }
        }
        file_index -= 1;
    }
}

fn convert_disk_content_file_to_block(disk_content: Vec<(u32, Option<u64>)>) -> Vec<Option<u64>> {
    let mut disk_content_block: Vec<Option<u64>> = Vec::new();

    for (num, id) in disk_content {
        for _ in 0..num {
            disk_content_block.push(id);
        }
    }

    disk_content_block
}

fn part2(disk_map: &String) {
    let mut disk_content = get_disk_content_file(&disk_map);
    compress_disk_content_file(&mut disk_content);
    let disk_content_block = convert_disk_content_file_to_block(disk_content);
    let checksum: u128 = calculate_checksum(&disk_content_block);
    println!("Part 2 Answer: {}", checksum);
}

fn read_file() -> Result<String> {
    let mut disk_map: String = String::new();
    let file = File::open(format!("{}/inputs/day_9.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        disk_map = line;
    }

    Ok(disk_map)
}
