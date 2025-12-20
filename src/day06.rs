//! Day 6: Trash Compactor
//!
//! ## Problem Description
//!
//! Part 1: Parse cephalopod math problems arranged vertically in columns.
//! Each column is a problem with numbers stacked vertically and an operator
//! at the bottom. Problems are separated by columns of only spaces.
//!
//! Part 2: Cephalopod math is read right-to-left by character column.
//! Each character column forms digits that make numbers top-to-bottom.
//!
//! ## Solution Approach
//!
//! **Part 1**: Parse the grid as whitespace-separated tokens. Each token
//! column represents a problem. Read top to bottom for numbers and operator.
//!
//! **Part 2**: Parse character by character. Read right-to-left by character
//! position, forming numbers from consecutive digit characters top-to-bottom.
//!
//! **Complexity**: O(rows * cols) for parsing and processing the grid.

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: char,
}

fn parse_input_as_tokens(input: &str) -> Vec<Vec<String>> {
    // Parse input into a grid of whitespace-separated tokens
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_cols = lines
        .iter()
        .map(|line| line.split_whitespace().count())
        .max()
        .unwrap_or(0);

    let mut grid: Vec<Vec<String>> = vec![];
    for line in &lines {
        let mut tokens: Vec<String> =
            line.split_whitespace().map(|s| s.to_string()).collect();

        // Pad with empty strings to max columns
        while tokens.len() < max_cols {
            tokens.push(String::new());
        }
        grid.push(tokens);
    }

    grid
}

fn extract_problems_part1(input: &str) -> Vec<Problem> {
    let grid = parse_input_as_tokens(input);
    if grid.is_empty() {
        return vec![];
    }

    let num_cols = grid[0].len();
    let mut problems = vec![];

    for col in 0..num_cols {
        // Collect numbers and operator from this column
        let mut numbers = vec![];
        let mut operator = ' ';

        for row in &grid {
            let cell = &row[col];
            if cell.is_empty() {
                continue;
            }

            if cell == "*" || cell == "+" {
                operator = cell.chars().next().unwrap();
            } else if let Ok(num) = cell.parse::<i64>() {
                numbers.push(num);
            }
        }

        if !numbers.is_empty() && operator != ' ' {
            problems.push(Problem { numbers, operator });
        }
    }

    problems
}

fn parse_input_as_chars(input: &str) -> Vec<Vec<char>> {
    // Parse input as a character grid
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    let max_width = lines.iter().map(|line| line.len()).max().unwrap();

    let mut grid: Vec<Vec<char>> = vec![];
    for line in &lines {
        let mut row: Vec<char> = line.chars().collect();
        // Pad with spaces to max width
        while row.len() < max_width {
            row.push(' ');
        }
        grid.push(row);
    }

    grid
}

fn extract_problems_part2(input: &str) -> Vec<Problem> {
    let grid = parse_input_as_chars(input);
    if grid.is_empty() {
        return vec![];
    }

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // Find problem groups: consecutive non-space columns separated by all-space columns
    let mut col_groups: Vec<Vec<usize>> = vec![];
    let mut current_group: Vec<usize> = vec![];

    for col in 0..num_cols {
        let all_spaces = grid.iter().all(|row| row[col] == ' ');
        if all_spaces {
            if !current_group.is_empty() {
                col_groups.push(current_group.clone());
                current_group.clear();
            }
        } else {
            current_group.push(col);
        }
    }
    if !current_group.is_empty() {
        col_groups.push(current_group);
    }

    // Process each group right-to-left (reverse the groups)
    col_groups.reverse();

    let mut problems = vec![];

    for group in col_groups {
        // Process columns RTL; each column (top-to-bottom) forms one number
        let mut numbers = vec![];
        let mut operator = ' ';

        for &col in group.iter().rev() {
            // Read this column top-to-bottom (excluding operator row) to form one number
            let mut digit_string = String::new();

            for row in 0..(num_rows - 1) {
                let ch = grid[row][col];
                if ch.is_ascii_digit() {
                    digit_string.push(ch);
                }
                // Spaces are ignored
            }

            // Check if this column has the operator
            let ch_op = grid[num_rows - 1][col];
            if ch_op == '*' || ch_op == '+' {
                operator = ch_op;
            }

            // Parse the number from this column
            if !digit_string.is_empty() {
                if let Ok(num) = digit_string.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        if !numbers.is_empty() && operator != ' ' {
            problems.push(Problem { numbers, operator });
        }
    }

    problems
}

fn solve_problem(problem: &Problem) -> i64 {
    if problem.numbers.is_empty() {
        return 0;
    }

    let mut result = problem.numbers[0];
    for &num in &problem.numbers[1..] {
        match problem.operator {
            '+' => result += num,
            '*' => result *= num,
            _ => {}
        }
    }
    result
}

pub fn part_one(input: &str) -> i64 {
    let problems = extract_problems_part1(input);
    problems.iter().map(solve_problem).sum()
}

pub fn part_two(input: &str) -> i64 {
    let problems = extract_problems_part2(input);
    problems.iter().map(solve_problem).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 4277556);
        assert_eq!(part_two(&input), 3263827);
    }
}
