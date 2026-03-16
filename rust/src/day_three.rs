pub struct DayResult {
    pub total: u32,
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult { total: 0 };
    let input_rows = input.split("\r\n");
    for row in input_rows.into_iter() {
        let rating = find_joltage_rating(&(row.to_string())) as u32;
        if rating == 0 {
            panic!("Row total was zero: {:?}", row);
        }
        result.total += rating;
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

fn find_joltage_rating(input: &String) -> u16 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_case_one() {
        assert_eq!(98, run(&"987654321111111".to_string()).total);
    }
    #[test]
    fn sample_case_two() {
        assert_eq!(89, run(&"811111111111119".to_string()).total);
    }
    #[test]
    fn sample_case_three() {
        assert_eq!(78, run(&"234234234234278".to_string()).total);
    }
    #[test]
    fn sample_case_four() {
        assert_eq!(92, run(&"818181911112111".to_string()).total);
    }
    #[test]
    fn sample_case_five() {
        assert_eq!(99, run(&"4536334834426354666653548333733764333733325524343558454435334323934332564372337559263534255825234943".to_string()).total);
    }
    #[test]
    fn sample_case_six() {
        assert_eq!(89, run(&"11123456789".to_string()).total);
    }
    #[test]
    fn sample_case_seven() {
        assert_eq!(91, run(&"11123456789111".to_string()).total);
    }
    #[test]
    fn string_to_arr() {
        assert_eq!(vec![1, 2, 3, 4], string_to_digit_vec(&"1234".to_string()))
    }
}
