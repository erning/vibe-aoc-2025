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

    // Find column groups (continuous non-space regions separated by space columns)
    // A column is considered "space" if it contains only spaces across all data rows
    let mut column_groups: Vec<(usize, usize)> = Vec::new(); // (start, end) inclusive

    let mut in_group = false;
    let mut group_start = 0;

    for col in 0..width {
        // Check if this column is all spaces across all data rows (excluding ops line)
        let is_space_col = (0..height - 1).all(|row| {
            let line = if row < lines.len() { lines[row] } else { "" };
            if col < line.len() {
                let ch = line.chars().nth(col).unwrap_or(' ');
                ch == ' '
            } else {
                true
            }
        });

        if !is_space_col {
            if !in_group {
                group_start = col;
                in_group = true;
            }
        } else if in_group {
            column_groups.push((group_start, col - 1));
            in_group = false;
        }
    }
    if in_group {
        column_groups.push((group_start, width - 1));
    }

    // Find operator positions
    let mut operators = Vec::new();
    for (col, ch) in ops_line.chars().enumerate() {
        if ch == '*' || ch == '+' {
            operators.push((col, ch));
        }
    }

    // Match column groups to operators by finding which operator is closest to each group
    let mut problems = Vec::new();

    for (group_idx, (start_col, end_col)) in column_groups.iter().enumerate()
    {
        // Find which operator corresponds to this group
        let operator = if group_idx < operators.len() {
            operators[group_idx].1
        } else {
            '+'
        };

        let mut numbers = Vec::new();

        if right_to_left {
            // Part 2: Read columns in reverse order, extract digits top-to-bottom for each column
            let col_order: Vec<usize> =
                (*start_col..=*end_col).rev().collect();

            for col in col_order {
                let mut num_str = String::new();

                // Extract the digit from this column across all rows
                for row in 0..height - 1 {
                    let line =
                        if row < lines.len() { lines[row] } else { "" };
                    if col < line.len() {
                        let ch = line.chars().nth(col).unwrap_or(' ');
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                        }
                    }
                }

                if !num_str.is_empty() {
                    if let Ok(num) = num_str.parse::<u64>() {
                        numbers.push(num);
                    }
                }
            }
        } else {
            // Part 1: Read each row as a whole, extract all digits in order
            for row in 0..height - 1 {
                let line = if row < lines.len() { lines[row] } else { "" };

                // Extract the substring for this problem group
                let start_idx = *start_col;
                let end_idx = (*end_col + 1).min(line.len());

                let substring = if start_idx < line.len() {
                    &line[start_idx..end_idx]
                } else {
                    ""
                };

                // Parse the number from this substring (collect all digits)
                let mut num_str = String::new();
                for ch in substring.chars() {
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                    }
                    // Skip spaces and continue - don't stop at first space
                }

                if !num_str.is_empty() {
                    if let Ok(num) = num_str.parse::<u64>() {
                        numbers.push(num);
                    }
                }
            }
        }

        if !numbers.is_empty() {
            problems.push(Problem { numbers, operator });
        }
    }

    // If reading right-to-left, reverse the order of problems
    if right_to_left {
        problems.reverse();
    }

    problems
}

fn solve_problem(numbers: Vec<u64>, operator: char) -> u64 {
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
    let problems = extract_problems(&lines, false);

    problems
        .iter()
        .map(|p| solve_problem(p.numbers.clone(), p.operator))
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let lines = parse_input(input);
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
