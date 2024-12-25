mod day_1;
mod day_2;

pub fn run(day_num: u8) {
    match day_num {
        1 => day_1::run(),
        2 => day_2::run(),
        _ => println!("Day number is not supported (yet)!"),
    }
}
