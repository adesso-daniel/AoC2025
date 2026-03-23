use std::ops::Range;

use crate::common::parse_range;

pub struct DayResult {
    pub sum: u64,
    pub sum_extra: u64,
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        sum: 0,
        sum_extra: 0,
    };

    let ranges: Vec<&str> = input.split(",").collect();
    for range in ranges {
        let r = parse_range(range);
        let range_result = check_range(&r);
        result.sum += range_result;
        let range_result = check_range_extra(&r);
        result.sum_extra += range_result;
    }

    return result;
}

fn string_into_chunks(input: &String, size: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let num_parts = (input.len() as f32 / size as f32).ceil() as usize;

    if num_parts <= 1 {
        return result;
    }

    for n in 0..num_parts {
        let part: String = input[(n * size)..(n * size + size)].to_string();

        result.push(part);
    }

    result
}

fn check_range(range: &Range<u64>) -> u64 {
    let mut result = 0;
    let mut invalid_ids: Vec<u64> = vec![];

    for i in range.start..=range.end {
        let i_str = i.to_string();
        let len = i_str.len();
        if len > 0 && len % 2 != 0 {
            // skip numbers of odd length
            continue;
        }
        let len_half = len / 2;
        let split = i_str.split_at(len_half);
        if split.0 != split.1 {
            continue;
        }
        invalid_ids.push(i);
        result += i;
    }

    result
}

fn check_range_extra(range: &Range<u64>) -> u64 {
    let mut result = 0;
    let mut invalid_ids: Vec<u64> = vec![];

    for i in range.start..=range.end {
        let i_str = i.to_string();
        let len = i_str.len();

        if len == 1 {
            continue;
        }

        // quick check with size 1
        let first_char = i_str.chars().nth(0).expect("Should be a single digit");
        if i_str.chars().all(|c| c == first_char) {
            result += i;
            invalid_ids.push(i);
            continue;
        }

        // split the string in increasingly small parts, starting with two parts down to pairs of digits
        for part_size in 2..len {
            if len % part_size != 0 {
                // the string is not divisable into equal parts, skip it
                continue;
            }

            let chars = string_into_chunks(&i_str, part_size);

            // compare the first split to the rest of the splits
            if chars.is_empty() || !chars.iter().all(|p| *p == chars[0]) {
                continue;
            }

            invalid_ids.push(i);
            result += i;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_two_tests_return_expected_result() {
        assert_eq!(
            1227775554,
            run(&"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                .to_string()).sum,
        );
        assert_eq!(
            4174379265,
            run(&"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
                .to_string()).sum_extra,
        );
    }

    #[test]
    fn check_range_should_return_sum_of_invalid_ids() {
        assert_eq!(33, check_range(&(11..22))); // 11 AND 22
    }

    #[test]
    fn check_range_extra_should_not_double_count() {
        assert_eq!(222222, check_range_extra(&(222220..222224))); // only 222222
    }

    #[test]
    fn check_range_extra_should_not_count_single_digits() {
        assert_eq!(11, check_range_extra(&(2..17))); // only 11
    }

    #[test]
    fn string_into_chunks_returns_expected_result() {
        let i_str = "123456".to_string();
        let part_size = 2;
        let chars = string_into_chunks(&i_str, part_size);
        assert_eq!(3, chars.len());
        assert_eq!("34", chars[1]);
    }

    #[test]
    fn string_into_chunks_three_digit_returns_expected_result() {
        let i_str = "999".to_string();
        let part_size = 1;
        let chars = string_into_chunks(&i_str, part_size);
        assert_eq!(3, chars.len());
        let comparison = chars.iter().all(|x| x == "9");
        assert_eq!(true, comparison);
    }
}
