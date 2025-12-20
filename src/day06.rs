//! Day 6: Trash Compactor
//!
//! ## Problem Description
//!
//! Part 1: Parse math problems arranged as columns with numbers at top.
//! Numbers are arranged horizontally within each group, separated by spaces.
//! Operator at the bottom. Calculate each problem, sum results.
//!
//! Part 2: Same problems but read groups from right-to-left.

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

fn extract_problems(lines: &[&str], right_to_left: bool) -> Vec<Problem> {
    if lines.is_empty() {
        return Vec::new();
    }

    let height = lines.len();
    let width = if height > 0 { lines[0].len() } else { 0 };
    let ops_line = if height > 0 { lines[height - 1] } else { "" };

    // Find operator positions in the last line
    let mut operator_positions = Vec::new();
    for (col, ch) in ops_line.chars().enumerate() {
        if ch == '*' || ch == '+' {
            operator_positions.push((col, ch));
        }
    }

    if right_to_left {
        operator_positions.reverse();
    }

    // For each operator position, extract the numbers above it in that group
    let mut problems = Vec::new();

    for (op_idx, (_op_col, operator)) in operator_positions.iter().enumerate() {
        let mut numbers = Vec::new();

        // Find the extent of this problem's columns (left and right bounds)
        let left_bound = if op_idx > 0 {
            operator_positions[op_idx - 1].0 + 1
        } else {
            0
        };
        let right_bound = if op_idx < operator_positions.len() - 1 {
            operator_positions[op_idx + 1].0 - 1
        } else {
            width - 1
        };

        // Extract numbers from this problem region, reading each row left-to-right
        for row in 0..height - 1 {
            let line = if row < lines.len() { lines[row] } else { "" };
            let mut num_str = String::new();

            let start = if right_to_left {
                right_bound
            } else {
                left_bound
            };
            let end = if right_to_left {
                left_bound
            } else {
                right_bound
            };

            if right_to_left {
                for col in (end..=start).rev() {
                    if col < line.len() {
                        let ch = line.chars().nth(col).unwrap();
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                        }
                    }
                }
            } else {
                for col in start..=end {
                    if col < line.len() {
                        let ch = line.chars().nth(col).unwrap();
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                        }
                    }
                }
            }

            if !num_str.is_empty() {
                if let Ok(num) = num_str.parse::<u64>() {
                    numbers.push(num);
                }
            }
        }

        if !numbers.is_empty() {
            problems.push(Problem {
                numbers,
                operator: *operator,
            });
        }
    }

    problems
}

fn solve_problem(
    numbers: Vec<u64>,
    operator: char,
) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> u64 {
    let lines = parse_input(input);
    // Part 1 reads left-to-right
    let problems = extract_problems(&lines, false);

    problems
        .iter()
        .map(|p| solve_problem(p.numbers.clone(), p.operator))
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let lines = parse_input(input);
    // Part 2 reads right-to-left
    let problems = extract_problems(&lines, true);

    problems
        .iter()
        .map(|p| solve_problem(p.numbers.clone(), p.operator))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 4277556);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(6);
        assert_eq!(part_two(&input), 3263827);
    }
}
