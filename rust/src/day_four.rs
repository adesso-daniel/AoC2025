const ROLL: u8 = 1;
const SPACE: u8 = 0;

pub struct DayResult {
    pub total: u64,
    pub part_two_total: u64,
}

struct IterationResult {
    num_accessible_rolls: u64,
    rows: Vec<Vec<u8>>,
}

pub fn run(input: &String) -> DayResult {
    let mut result: DayResult = DayResult {
        total: 0,
        part_two_total: 0,
    };

    let rows = input_to_vectors(&input);
    let one_result = part_one(rows);
    result.total = one_result.num_accessible_rolls as u64;
    result.part_two_total = one_result.num_accessible_rolls as u64;
    let mut one_result = one_result.rows;
    loop {
        let two_result = part_one(one_result);
        if two_result.num_accessible_rolls == 0 {
            break;
        }
        result.part_two_total += two_result.num_accessible_rolls;
        one_result = two_result.rows;
    }
    result
}

fn string_to_digit_vec(input: &String) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for x in input.replace("\r", "").chars().into_iter() {
        let int = match x {
            '@' => ROLL,
            _ => SPACE,
        };
        result.push(int);
    }

    result
}

fn input_to_vectors(input: &String) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    let rows: Vec<&str> = input.split("\n").collect();

    for row in rows.iter() {
        let int_row = string_to_digit_vec(&row.to_string());
        result.push(int_row);
    }

    result
}

/// Counts the number of ROLL in the vector. It is expected that roll_surroundings contains at least one ROLL
fn count_surrounding_rolls(roll_surroundings: Vec<u8>) -> u8 {
    let mut sum: u8 = 0;
    for f in roll_surroundings.iter() {
        sum += f;
    }
    sum - 1
}

/// Checks the number of accessible rolls and returns a new map with those rolls removed
fn part_one(rows: Vec<Vec<u8>>) -> IterationResult {
    let num_rows = rows.len();
    let row_width = rows[0].len();

    let mut remaining_rows = rows;
    let mut num_accessible_rolls = 0;

    // skip edge cases for now
    for y in 0..num_rows {
        for x in 0..row_width {
            let x_left_clamped = i16::max(0, x as i16 - 1) as usize;
            let x_right_clamped = usize::min(row_width - 1, x + 1);
            let current = &remaining_rows[y][x_left_clamped..=x_right_clamped];

            if remaining_rows[y][x] == SPACE {
                continue;
            }
            let above = if y as i16 - 1 < 0 {
                &[0, 0, 0]
            } else {
                &remaining_rows[y - 1][x_left_clamped..=x_right_clamped]
            };
            let below = if y + 1 > num_rows - 1 {
                &[0, 0, 0]
            } else {
                &remaining_rows[y + 1][x_left_clamped..=x_right_clamped]
            };
            let roll_surroundings = [above, current, below].concat();
            let sum = count_surrounding_rolls(roll_surroundings);
            if sum < 4 {
                num_accessible_rolls += 1;
                remaining_rows[y][x] = SPACE;
            }
        }
    }

    IterationResult {
        num_accessible_rolls,
        rows: remaining_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_zero() {
        let vec_input: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let result = count_surrounding_rolls(vec_input);

        assert_eq!(8, result);
    }
    #[test]
    fn should_return_one() {
        let vec_input: Vec<u8> = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
        let result = count_surrounding_rolls(vec_input);

        assert_eq!(0, result);
    }

    #[test]
    fn should_solve_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        let result = run(&input);

        assert_eq!(13, result.total);
    }

    #[test]
    fn should_solve_part_two_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .to_string();
        let result = run(&input);

        assert_eq!(43, result.total);
    }
}
