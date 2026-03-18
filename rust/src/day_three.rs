pub struct DayResult {
    pub total: u64,
    pub override_total: u64,
}

struct ValueAndIndex {
    pub value: u16,
    pub index: usize,
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        override_total: 0,
    };
    let input_rows = input.split("\r\n");
    for row in input_rows.into_iter() {
        let rating = find_joltage_rating(&row.to_string());
        result.total += rating as u64;
        let rating = find_joltage_rating_override(&row.to_string(), 12);
        result.override_total += rating;
    }

    result
}

fn string_to_digit_vec(input: &String) -> Vec<u16> {
    let mut result: Vec<u16> = vec![];

    for x in input.chars().into_iter() {
        let int = match x.to_string().parse::<u16>() {
            Ok(value) => value,
            Err(error) => {
                panic!("Should be a single digit: {:?}. Error: {}", x, error);
            }
        };
        result.push(int);
    }

    result
}

/// Transforms the vector into a single number
fn vec_to_number(digits: Vec<u16>) -> u64 {
    let mut result: u64 = 0;
    let length = digits.len() - 1;
    for i in 0..=length {
        result += digits[length - i] as u64 * 10u64.pow((i) as u32);
    }
    result
}

fn first_index_of_max_digit(digits: &Vec<u16>) -> ValueAndIndex {
    let mut result: ValueAndIndex = ValueAndIndex { value: 0, index: 0 };
    for i in 0..digits.len() {
        let digit = digits[i];
        if digit > result.value {
            result.value = digit;
            result.index = i;
        }
    }
    result
}

pub fn find_joltage_rating(input: &String) -> u16 {
    let digits = string_to_digit_vec(input);

    let mut id_left: usize = 0;

    let mut max = 0;

    for val_left in digits.iter() {
        if id_left == input.len() - 1 {
            break;
        }
        for r in ((id_left + 1)..input.len()).rev() {
            let id_right = r;
            let val_right = digits[id_right];

            let value = val_left * 10 + val_right;
            if value > max {
                max = value;
            }
            if value == 99 {
                return 99;
            }
        }
        id_left += 1;
    }

    max
}

pub fn find_joltage_rating_override(input: &String, goal_length: usize) -> u64 {
    let digits = string_to_digit_vec(input);

    let mut starting_idx: usize = 0;
    let mut largest_digits: Vec<u16> = vec![];
    for i in 0..digits.len() {
        if largest_digits.len() == goal_length {
            break;
        }

        let upper_bound = digits.len() - goal_length + i;
        let range = digits[starting_idx..=upper_bound].to_vec();
        let max = first_index_of_max_digit(&range);

        largest_digits.push(max.value);
        starting_idx = starting_idx + max.index + 1;
    }

    let result: u64 = vec_to_number(largest_digits);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_largest_number_and_index() {
        let digits = vec![4, 1, 3, 2, 1, 9, 1, 1];
        let result = first_index_of_max_digit(&digits);
        let expected = ValueAndIndex { value: 9, index: 5 };

        assert_eq!(expected.value, result.value);
        assert_eq!(expected.index, result.index);
    }

    #[test]
    fn should_transform_array_into_number() {
        let digits = vec![4, 3, 2, 1, 1, 1];
        let result = vec_to_number(digits);

        assert_eq!(432111, result);
    }

    #[test]
    fn find_rating_should_return_expected_joltage_1() {
        assert_eq!(
            98,
            find_joltage_rating_override(&"987654321111111".to_string(), 2)
        );
        assert_eq!(
            987,
            find_joltage_rating_override(&"987654321111111".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_2() {
        assert_eq!(89, find_joltage_rating_override(&"811119".to_string(), 2));
        assert_eq!(
            819,
            find_joltage_rating_override(&"811111111111119".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_3() {
        assert_eq!(
            78,
            find_joltage_rating_override(&"234234234234278".to_string(), 2)
        );
        assert_eq!(
            478,
            find_joltage_rating_override(&"234234234234278".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_4() {
        assert_eq!(
            92,
            find_joltage_rating_override(&"818181911112111".to_string(), 2)
        );
        assert_eq!(
            921,
            find_joltage_rating_override(&"818181911112111".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_5() {
        assert_eq!(
            91,
            find_joltage_rating_override(&"11123456789111".to_string(), 2)
        );
        assert_eq!(
            911,
            find_joltage_rating_override(&"11123456789111".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_6() {
        assert_eq!(
            89,
            find_joltage_rating_override(&"11123456789".to_string(), 2)
        );
        assert_eq!(
            789,
            find_joltage_rating_override(&"11123456789".to_string(), 3)
        );
    }

    #[test]
    fn find_rating_should_return_expected_joltage_7() {
        assert_eq!(99, find_joltage_rating_override(&"924485422333337443343544333335343282383236552934443422243282543242234343463222433722434532232242344".to_string(), 2));
        assert_eq!(998, find_joltage_rating_override(&"924485422333337443343544333335343282383236552934443422243282543242234343463222433722434532232242344".to_string(), 3));
        assert_eq!(
            9987,
            find_joltage_rating_override(&"98788987".to_string(), 4)
        );
        assert_eq!(
            9889,
            find_joltage_rating_override(&"78988789".to_string(), 4)
        );
        assert_eq!(9987, find_joltage_rating_override(&"924485422333337443343544333335343282383236552934443422243282543242234343463222433722434532232242344".to_string(), 4));
        assert_eq!(98788987, find_joltage_rating_override(&"924485422333337443343544333335343282383236552934443422243282543242234343463222433722434532232242344".to_string(), 8));
    }

    #[test]
    fn find_rating_should_return_expected_joltage_8() {
        assert_eq!(
            987654321111,
            find_joltage_rating_override(&"987654321111111".to_string(), 12)
        );
    }
    #[test]
    fn find_rating_should_return_expected_joltage_9() {
        assert_eq!(
            811111111119,
            find_joltage_rating_override(&"811111111111119".to_string(), 12)
        );
    }
    #[test]
    fn find_rating_should_return_expected_joltage_10() {
        assert_eq!(
            434234234278,
            find_joltage_rating_override(&"234234234234278".to_string(), 12)
        );
    }
    #[test]
    fn find_rating_should_return_expected_joltage_11() {
        assert_eq!(
            888911112111,
            find_joltage_rating_override(&"818181911112111".to_string(), 12)
        );
    }
    #[test]
    fn find_rating_should_return_expected_joltage_12() {
        assert_eq!(
            89,
            find_joltage_rating_override(&"818283511112119".to_string(), 2)
        );
    }

    #[test]
    fn string_to_arr() {
        assert_eq!(vec![1, 2, 3, 4], string_to_digit_vec(&"1234".to_string()))
    }
}
