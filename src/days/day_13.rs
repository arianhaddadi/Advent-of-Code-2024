use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader, Result},
    u64,
};

#[derive(Copy, Clone)]
struct Button {
    x_inc: u64,
    y_inc: u64,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Button(x_inc: {}, y_inc: {})", self.x_inc, self.y_inc)
    }
}

struct Machine {
    a_button: Button,
    b_button: Button,
    prize: (u64, u64),
}

impl Machine {
    fn cal_min_token(&self) -> i64 {
        let determinant = (self.a_button.x_inc * self.b_button.y_inc) as i64
            - (self.a_button.y_inc * self.b_button.x_inc) as i64;
        if determinant == 0 {
            return 0;
        }
        let determinant_a = (self.prize.0 * self.b_button.y_inc) as i64
            - (self.b_button.x_inc * self.prize.1) as i64;
        let determinant_b = (self.a_button.x_inc * self.prize.1) as i64
            - (self.prize.0 * self.a_button.y_inc) as i64;
        if determinant_a % determinant != 0 || determinant_b % determinant != 0 {
            return 0;
        }
        let num_of_a = determinant_a / determinant;
        let num_of_b = determinant_b / determinant;
        num_of_a * 3 + num_of_b
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Machine(a_button: {}, b_button: {}, prize: ({}, {}))",
            self.a_button, self.b_button, self.prize.0, self.prize.1
        )
    }
}

pub fn run() {
    let machines = read_file();
    match machines {
        Ok(mut machines) => {
            part1(&machines);
            part2(&mut machines);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}

fn part1(machines: &Vec<Machine>) {
    let mut tokens = 0;

    for machine in machines.iter() {
        tokens += machine.cal_min_token();
    }

    println!("Part 1 Answer: {}", tokens);
}

fn part2(machines: &mut Vec<Machine>) {
    let mut tokens = 0;

    for machine in machines.iter_mut() {
        machine.prize = (
            machine.prize.0 + 10000000000000,
            machine.prize.1 + 10000000000000,
        );
        tokens += machine.cal_min_token();
    }

    println!("Part 2 Answer: {}", tokens);
}

fn read_file() -> Result<Vec<Machine>> {
    let mut machines: Vec<Machine> = Vec::new();

    let file = File::open(format!("{}/inputs/day_13.txt", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(file);

    let mut a_button = Button { x_inc: 0, y_inc: 0 };
    let mut b_button = Button { x_inc: 0, y_inc: 0 };

    for line in reader.lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }

        let line_sections = line.split_whitespace().collect::<Vec<&str>>();
        if vec![line_sections[0], line_sections[1]].join(" ") == "Button A:" {
            a_button = Button {
                x_inc: line_sections[2][2..line_sections[2].len() - 1]
                    .parse()
                    .unwrap(),
                y_inc: line_sections[3][2..].parse().unwrap(),
            };
            continue;
        }

        if vec![line_sections[0], line_sections[1]].join(" ") == "Button B:" {
            b_button = Button {
                x_inc: line_sections[2][2..line_sections[2].len() - 1]
                    .parse()
                    .unwrap(),
                y_inc: line_sections[3][2..].parse().unwrap(),
            };
            continue;
        }

        if line_sections[0] == "Prize:" {
            let prize = (
                line_sections[1][2..line_sections[1].len() - 1]
                    .parse()
                    .unwrap(),
                line_sections[2][2..].parse().unwrap(),
            );
            machines.push(Machine {
                a_button,
                b_button,
                prize,
            });
        }
    }

    Ok(machines)
}
