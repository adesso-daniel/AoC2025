use std::ops::RangeInclusive;

use crate::common::parse_range_inclusive;

pub struct DayResult {
    pub total: u64,
    pub part_two_total: u64,
}

struct Inputs {
    pub ranges: Vec<RangeInclusive<u64>>,
    pub numbers: Vec<u64>,
    pub lower_bound: u64,
    pub upper_bound: u64,
}

fn parse_inputs(input: &String) -> Inputs {
    let foo = input.replace("\r", "");
    let input_split: Vec<&str> = foo.split("\n\n").collect();
    let ranges: Vec<&str> = input_split[0].split('\n').collect();
    let numbers: Vec<&str> = input_split[1].split('\n').collect();

    let mut result: Inputs = Inputs {
        ranges: vec![],
        numbers: vec![],
        lower_bound: u64::MAX,
        upper_bound: 0,
    };

    for range in ranges {
        let r = parse_range_inclusive(range);
        let start = *r.start();
        let end = *r.end();
        result.ranges.push(r);
        if start < result.lower_bound {
            result.lower_bound = start;
        }
        if end > result.upper_bound {
            result.upper_bound = end;
        }
    }

    for number in numbers {
        if number.trim().len() == 0 {
            continue;
        }
        let n = number.parse::<u64>().expect("Should be a number");
        result.numbers.push(n);
    }

    result
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        part_two_total: 0,
    };

    let inputs = parse_inputs(input);
    for number in inputs.numbers {
        if number < inputs.lower_bound {
            continue;
        }
        if number > inputs.upper_bound {
            continue;
        }

        for range in inputs.ranges.iter() {
            if range.contains::<u64>(&number) {
                result.total += 1;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_input() {
        let input = "3-5

4"
        .to_string();

        let parsed = parse_inputs(&input);

        assert_eq!(3..=5, parsed.ranges[0]);
        assert_eq!(4, parsed.numbers[0]);
    }

    #[test]
    fn should_solve_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        .to_string();

        assert_eq!(3, run(&input).total);
    }
}
