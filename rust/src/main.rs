use std::{env, fs};

use aoc2025::{day_five, day_four, day_one, day_three, day_two};

// Input is expected to be a txt file with a numeric name eg. "12.txt"
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = fs::read_to_string(path).expect("Should have been able to read the file");
    let unix_path = path.replace('\\', "/");
    let filename = unix_path
        .split("/")
        .last()
        .expect("should be a path")
        .split('.')
        .nth(0);

    match filename {
        Some("1") => {
            let result = day_one::run(input, 50);
            println!(
                "day one! Landed on zero {} times, passed zero {} times. Total {}",
                result.landed_on_zero,
                result.passed_zero,
                result.landed_on_zero + result.passed_zero
            );
        }
        Some("2") => {
            let result = day_two::run(&input);
            println!("Day two: {} & {}", result.sum, result.sum_extra);
        }
        Some("3") => {
            let result = day_three::run(&input);
            println!(
                "Day three normal: {}, override: {}",
                result.total, result.override_total
            );
        }
        Some("4") => {
            let result = day_four::run(&input);
            println!("Day four: {} & {}", result.total, result.part_two_total);
        }
        Some("5") => {
            let result = day_five::run(&input);
            println!("Day five: {} & {}", result.total, result.part_two_total);
        }
        _ => {
            println!("Day '{:?}' not implemented", filename)
        }
    }
}

mod tests {

    #[test]
    fn can_read_input_argument() {}
}
