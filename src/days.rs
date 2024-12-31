mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub fn run(day_num: u8) {
    match day_num {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        _ => println!("Day number is not supported (yet)!"),
    }
}
