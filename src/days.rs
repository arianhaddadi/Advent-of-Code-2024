mod day_1;

pub fn run(day_num: u8) {
    match day_num {
        1 => day_1::run(),
        _ => println!("Day number is not supported (yet)!"),
    }
}
