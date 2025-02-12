mod days;

fn main() {
    println!("************Advent of Code 2024******************");
    for day_num in 1..26 {
        days::run(day_num);
    }
    println!("************************************************");
}
