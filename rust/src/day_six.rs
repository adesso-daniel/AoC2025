use std::usize;

pub struct DayResult {
    pub total: u64,
    pub part_two_total: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Invalid,
    Add,
    Mult,
}

#[derive(Debug)]
struct Problem {
    start_index: usize,
    operator: Operator,
    numbers: Vec<u32>,
    human_numbers: Vec<u32>,
}

impl PartialEq for Problem {
    fn eq(&self, other: &Self) -> bool {
        self.start_index == other.start_index
            // && self.Width == other.Width
            && self.operator == other.operator
            && self.numbers == other.numbers
            && self.human_numbers == other.human_numbers
    }
}
impl Eq for Problem {}

fn initialize_problems(input: &Vec<&str>) -> Vec<Problem> {
    let mut result = vec![];
    for i in 0..input.len() {
        let maybe_operator = input[i].trim();
        if maybe_operator.len() == 0 {
            continue;
        }
        let operator = match maybe_operator {
            "+" => Operator::Add,
            "*" => Operator::Mult,
            _ => {
                println!("Invalid operator '{}'", maybe_operator);
                Operator::Invalid
            }
        };

        result.push(Problem {
            operator,
            start_index: i,
            numbers: vec![],
            human_numbers: vec![],
        });
    }
    result
}

fn addition(values: &Vec<u32>) -> u64 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i] as u64;
    }
    result
}

fn product(values: &Vec<u32>) -> u64 {
    let mut result = values[0] as u64;
    for i in 1..values.len() {
        result *= values[i] as u64;
    }
    result
}

fn rows_to_char_arrays(input: &String) -> Vec<Vec<&str>> {
    let all_rows: Vec<Vec<&str>> = input
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|m| m.split("").collect::<Vec<&str>>()[1..=m.len()].to_vec())
        .collect();
    all_rows
}

/// Parses numbers from all_rows and returns a new Vec<Problem>
fn parse_numbers(all_rows: Vec<Vec<&str>>, problems: Vec<Problem>) -> Vec<Problem> {
    let mut results: Vec<Problem> = vec![];
    let num_problems = problems.len();
    for i in 0..num_problems {
        let problem = &problems[i];
        let next_problem = if i + 1 >= problems.len() {
            &Problem {
                start_index: all_rows[0].len(),
                operator: Operator::Invalid,
                numbers: vec![],
                human_numbers: vec![],
            }
        } else {
            &problems[i + 1]
        };

        let numbers = parse_cephalopods(&all_rows, problem, next_problem);
        let human_numbers = parse_humans(&all_rows, problem, next_problem);

        let operator = problem.operator.clone();
        results.push(Problem {
            start_index: problem.start_index,
            operator,
            numbers,
            human_numbers,
        })
    }
    results
}

fn parse_humans(all_rows: &Vec<Vec<&str>>, problem: &Problem, next_problem: &Problem) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let num_rows = all_rows.len();
    // step through the rows top to bottom
    for row_index in 0..num_rows - 1 {
        // get the digits for the problem from the full set of characters
        let chars = all_rows[row_index][problem.start_index..next_problem.start_index].concat();
        let digit: u32 = match chars.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => 0,
        };

        if digit != 0 {
            result.push(digit);
        }
    }

    result
}

fn parse_cephalopods(
    all_rows: &Vec<Vec<&str>>,
    problem: &Problem,
    next_problem: &Problem,
) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let num_rows = all_rows.len();
    let width = next_problem.start_index - problem.start_index;
    // step through problem columns, from right to left
    for j in 0..width {
        let col = width - 1 - j;
        let mut number = 0;
        let mut digit_significance = 0u32;
        // step through the rows top to bottom
        for r in 1..num_rows {
            let row_index = num_rows - 1 - r; // reverse index
            // get the digits for the problem from the full set of characters
            let chars = all_rows[row_index][problem.start_index..next_problem.start_index].to_vec();
            let digit: u32 = if col >= chars.len() {
                0
            } else {
                match chars[col].parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                }
            };
            let multiplier = 10u32.pow(digit_significance);
            number += digit as u32 * multiplier;
            if digit != 0 {
                digit_significance += 1;
            }
        }
        if number != 0 {
            result.push(number);
        }
    }
    result
}

fn calculate(numbers: &Vec<u32>, operator: Operator) -> u64 {
    match operator {
        Operator::Add => addition(&numbers),
        Operator::Mult => product(&numbers),
        Operator::Invalid => 0,
    }
}

fn sum_part_one(problems: &Vec<Problem>) -> u64 {
    let mut result = 0;
    for i in 0..problems.len() {
        let math = calculate(&problems[i].human_numbers, problems[i].operator);

        result += math;
    }
    result
}

fn sum_part_two(problems: &Vec<Problem>) -> u64 {
    let mut result = 0;
    for i in 0..problems.len() {
        let math = calculate(&problems[i].numbers, problems[i].operator);

        result += math;
    }
    result
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        part_two_total: 0,
    };
    let all_rows = rows_to_char_arrays(&input);

    let columns: Vec<Problem> = initialize_problems(&all_rows[all_rows.len() - 1]);
    let problems = parse_numbers(all_rows, columns);

    result.total = sum_part_one(&problems);
    result.part_two_total = sum_part_two(&problems);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();
        let result = run(&input);

        assert_eq!(4277556, result.total)
    }

    // test with four input rows
    #[test]
    fn should_solve_example_2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
  1 1   1   1  
*   +   *   +  "
            .to_string();
        let result = run(&input);

        assert_eq!(4277558, result.total)
    }

    #[test]
    fn should_solve_example_part_two() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .to_string();
        let result = run(&input);

        assert_eq!(3263827, result.part_two_total)
    }

    #[test]
    fn should_split_input_to_char_arrays() {
        let input = "123 51 
 45 387
  6 215
*   +  "
            .to_string();
        let result = rows_to_char_arrays(&input);

        assert_eq!(
            vec![
                vec!["1", "2", "3", " ", "5", "1", " "],
                vec![" ", "4", "5", " ", "3", "8", "7"],
                vec![" ", " ", "6", " ", "2", "1", "5"],
                vec!["*", " ", " ", " ", "+", " ", " "]
            ],
            result
        )
    }

    #[test]
    fn should_initialize_columns() {
        let input = "383 3   43
734 979 57
682 813 49
 74 849 51
*   +   * "
            .to_string();
        let split_input = rows_to_char_arrays(&input);

        let result = initialize_problems(&split_input[split_input.len() - 1]);
        let expected_a = Problem {
            start_index: 0,
            operator: Operator::Mult,
            numbers: vec![],
            human_numbers: vec![],
        };
        let expected_b = Problem {
            start_index: 4,
            operator: Operator::Add,
            numbers: vec![],
            human_numbers: vec![],
        };
        let expected_c = Problem {
            start_index: 8,
            operator: Operator::Mult,
            numbers: vec![],
            human_numbers: vec![],
        };

        assert_eq!(vec![expected_a, expected_b, expected_c], result)
    }

    #[test]
    fn should_parse_numbers() {
        let all_rows = vec![
            vec!["3", "8", "3", " ", "3", " ", " ", " ", "4", "3"],
            vec!["7", "3", "4", " ", "9", "7", "9", " ", "5", "7"],
            vec!["6", "8", "2", " ", "8", "1", "3", " ", "4", "9"],
            vec![" ", "7", "4", " ", "8", "4", "9", " ", "5", "1"],
            vec!["*", " ", " ", " ", "+", " ", " ", " ", "*", " "],
        ];
        let problems = vec![
            Problem {
                start_index: 0,
                operator: Operator::Mult,
                numbers: vec![],
                human_numbers: vec![],
            },
            Problem {
                start_index: 4,
                operator: Operator::Add,
                numbers: vec![],
                human_numbers: vec![],
            },
            Problem {
                start_index: 8,
                operator: Operator::Mult,
                numbers: vec![],
                human_numbers: vec![],
            },
        ];

        let expected = vec![
            Problem {
                start_index: 0,
                operator: Operator::Mult,
                numbers: vec![3424, 8387, 376],
                human_numbers: vec![383, 734, 682, 74],
            },
            Problem {
                start_index: 4,
                operator: Operator::Add,
                numbers: vec![939, 714, 3988],
                human_numbers: vec![3, 979, 813, 849],
            },
            Problem {
                start_index: 8,
                operator: Operator::Mult,
                numbers: vec![3791, 4545],
                human_numbers: vec![43, 57, 49, 51],
            },
        ];

        let result = parse_numbers(all_rows, problems);
        assert_eq!(expected, result);
    }
}
