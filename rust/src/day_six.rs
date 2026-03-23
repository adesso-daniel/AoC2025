pub struct DayResult {
    pub total: u64,
    pub part_two_total: u64,
}

enum Operators {
    Invalid,
    Add,
    Mult,
}

fn row_to_numbers(input: &str) -> Vec<u64> {
    let num_arr: Vec<&str> = input.split(" ").collect();
    let mut result = vec![];

    for n in num_arr {
        if n.trim().len() == 0 {
            continue;
        }
        let number = n.trim().parse::<u64>().expect("Should be a number");
        result.push(number);
    }
    result
}

fn row_to_operators(input: &str) -> Vec<Operators> {
    let num_arr: Vec<&str> = input.split(" ").collect();
    let mut result = vec![];

    for n in num_arr {
        if n.trim().len() == 0 {
            continue;
        }
        let operator = match n.trim() {
            "+" => Operators::Add,
            "*" => Operators::Mult,
            error => {
                println!("Invalid operator {}", error);
                Operators::Invalid
            }
        };
        result.push(operator);
    }
    result
}

fn sum(values: &Vec<u64>) -> u64 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i];
    }
    result
}

fn prod(values: &Vec<u64>) -> u64 {
    let mut result = values[0];
    for i in 1..values.len() {
        result *= values[i];
    }
    result
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        part_two_total: 0,
    };
    let all_rows: Vec<&str> = input.split('\n').filter(|f| f.trim() != "").collect();

    let num_rows = all_rows.len() - 1;

    let mut numbers: Vec<Vec<u64>> = vec![];
    let mut operators: Vec<Operators> = vec![];

    for i in 0..all_rows.len() {
        if i == all_rows.len() - 1 {
            operators = row_to_operators(all_rows[i]);
            break;
        }

        numbers.push(row_to_numbers(all_rows[i]));
    }

    let num_problems = operators.len();

    for i in 0..num_problems {
        let mut column_numbers = vec![];

        for j in 0..num_rows {
            column_numbers.push(numbers[j][i]);
        }
        let math = match operators[i] {
            Operators::Add => sum(&column_numbers),
            Operators::Mult => prod(&column_numbers),
            Operators::Invalid => 0,
        };

        result.total += math;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_example() {
        let input = "123 328    51   64 
 45 64    387   23 
  6 98    215   314
*   +     *     +  "
            .to_string();
        let result = run(&input);

        assert_eq!(4277556, result.total)
    }

    // test with four input rows
    #[test]
    fn should_solve_example_2() {
        let input = "123 328    51   64 
 45 64    387   23 
  6 98    215   314
  1 1     1     1
*   +     *     +  "
            .to_string();
        let result = run(&input);

        assert_eq!(4277558, result.total)
    }

    #[test]
    fn should_solve_example_part_two() {
        let input = "123 328    51   64 
 45 64    387   23 
  6 98    215   314
*   +     *     +  "
            .to_string();
        let result = run(&input);

        assert_eq!(3263827, result.part_two_total)
    }
}
