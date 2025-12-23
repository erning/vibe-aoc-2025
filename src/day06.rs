//! Day 6: Trash Compactor
//!
//! Part 1: Parse math problems arranged in columns, compute grand total.
//! Part 2: Read numbers right-to-left (character by character) per cephalopod convention.
//!
//! ## Solution Approach
//!
//! **Input Format**: A 2D grid where:
//! - Each problem column has an operator at the bottom
//! - Numbers are arranged horizontally (left-to-right) in each row
//! - Spaces separate numbers within each row
//! - Separator columns (full columns of spaces) separate problems
//!
//! **Part 1 Strategy**:
//! - Find separator columns (all spaces in all data rows)
//! - Each problem spans from after the previous separator up to next separator
//! - Find the operator within each problem's column range
//! - Parse all numbers in each row and assign them to the problem they're in
//! - Apply operator and sum results
//!
//! **Part 2 Strategy**:
//! - Cephalopod math reads character columns right-to-left
//! - Each problem spans from after the previous separator up to next separator
//! - Read each character column top-to-bottom to form a number
//! - Group numbers by problem and apply operators (in reverse column order)
//!
//! **Complexity**: O(r * c) where r is rows and c is columns.

/// Parse input into a grid of characters
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

/// A parsed number with its column position
#[derive(Debug, Clone)]
struct NumberAt {
    value: u128,
    col_start: usize,
}

/// Parse a row into a list of numbers with their positions
fn parse_row(row: &[char]) -> Vec<NumberAt> {
    let mut numbers = Vec::new();
    let mut col = 0;

    while col < row.len() {
        while col < row.len() && !row[col].is_ascii_digit() {
            col += 1;
        }

        if col >= row.len() {
            break;
        }

        let start = col;
        while col < row.len() && row[col].is_ascii_digit() {
            col += 1;
        }

        let num_str: String = row[start..col].iter().collect();
        if let Ok(value) = num_str.parse::<u128>() {
            numbers.push(NumberAt {
                value,
                col_start: start,
            });
        }
    }

    numbers
}

/// Find all separator columns (columns that are all spaces in all data rows)
fn find_separators(grid: &[Vec<char>], num_data_rows: usize) -> Vec<usize> {
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut separators = Vec::new();

    for col in 0..max_cols {
        let mut is_separator = true;
        for row in grid.iter().take(num_data_rows) {
            if col < row.len() && row[col] != ' ' {
                is_separator = false;
                break;
            }
        }
        if is_separator {
            separators.push(col);
        }
    }

    separators
}

/// Define a problem's column range
#[derive(Debug)]
struct ProblemRange {
    col_start: usize,
    col_end: usize, // exclusive
    op: char,
}

/// Find all problem ranges using separators as boundaries
fn find_problem_ranges(
    grid: &[Vec<char>],
    num_data_rows: usize,
) -> Vec<ProblemRange> {
    let separators = find_separators(grid, num_data_rows);
    let op_row = &grid[grid.len() - 1];
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    let mut problems = Vec::new();

    // Start from column 0 (or after first separator if it's at 0)
    let mut start_col = 0;

    for &sep in &separators {
        // Problem spans from start_col to sep (exclusive)
        if start_col < sep {
            // Find operator in this range
            let mut op = '+';
            for col in start_col..sep {
                if col < op_row.len() {
                    let ch = op_row[col];
                    if ch == '+' || ch == '*' {
                        op = ch;
                        break;
                    }
                }
            }
            problems.push(ProblemRange {
                col_start: start_col,
                col_end: sep,
                op,
            });
        }
        start_col = sep + 1;
    }

    // Last problem from last separator to end
    if start_col < max_cols {
        let mut op = '+';
        for col in start_col..max_cols {
            if col < op_row.len() {
                let ch = op_row[col];
                if ch == '+' || ch == '*' {
                    op = ch;
                    break;
                }
            }
        }
        problems.push(ProblemRange {
            col_start: start_col,
            col_end: max_cols,
            op,
        });
    }

    problems
}

pub fn part_one(input: &str) -> u128 {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let num_data_rows = grid.len() - 1;
    let problems = find_problem_ranges(&grid, num_data_rows);

    // Parse all data rows into numbers
    let mut all_numbers: Vec<Vec<NumberAt>> = Vec::new();
    for row in grid.iter().take(num_data_rows) {
        all_numbers.push(parse_row(row));
    }

    // Assign each number to the problem whose range contains it
    let mut problem_numbers: Vec<Vec<u128>> =
        vec![Vec::new(); problems.len()];

    for row_numbers in &all_numbers {
        for num in row_numbers {
            // Find which problem range contains this number's start column
            for (idx, problem) in problems.iter().enumerate() {
                if num.col_start >= problem.col_start
                    && num.col_start < problem.col_end
                {
                    problem_numbers[idx].push(num.value);
                    break;
                }
            }
        }
    }

    // Compute results
    let mut total: u128 = 0;
    for (idx, problem) in problems.iter().enumerate() {
        let result: u128 = match problem.op {
            '+' => problem_numbers[idx].iter().sum(),
            '*' => problem_numbers[idx].iter().product(),
            _ => 0,
        };
        total += result;
    }

    total
}

pub fn part_two(input: &str) -> u128 {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let num_data_rows = grid.len() - 1;
    let problems = find_problem_ranges(&grid, num_data_rows);

    // Find max column width
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // For each character column, determine which problem it belongs to
    // Then read that column top-to-bottom to form a number
    let mut problem_numbers: Vec<Vec<(usize, u128)>> =
        vec![Vec::new(); problems.len()];

    for col in 0..max_cols {
        // Check if this column has any digits
        let mut has_digits = false;
        for row in grid.iter().take(num_data_rows) {
            if col < row.len() && row[col].is_ascii_digit() {
                has_digits = true;
                break;
            }
        }

        if !has_digits {
            continue;
        }

        // Find which problem range contains this column
        let mut problem_idx = None;
        for (idx, problem) in problems.iter().enumerate() {
            if col >= problem.col_start && col < problem.col_end {
                problem_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = problem_idx {
            // Read this column top-to-bottom to form a number
            let mut num_str = String::new();
            for row in grid.iter().take(num_data_rows) {
                if col < row.len() && row[col].is_ascii_digit() {
                    num_str.push(row[col]);
                }
            }

            if let Ok(num) = num_str.parse::<u128>() {
                problem_numbers[idx].push((col, num));
            }
        }
    }

    // Compute results - numbers are processed right-to-left within each problem
    let mut total: u128 = 0;
    for (idx, problem) in problems.iter().enumerate() {
        // Sort by column position in descending order (right-to-left)
        problem_numbers[idx].sort_by_key(|&(col, _)| std::cmp::Reverse(col));
        let numbers: Vec<u128> =
            problem_numbers[idx].iter().map(|&(_, n)| n).collect();

        let result: u128 = match problem.op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };
        total += result;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        // Part 1: 33210 + 490 + 4243455 + 401 = 4277556
        assert_eq!(part_one(&input), 4277556);
        // Part 2: 1058 + 3253600 + 625 + 8544 = 3263827
        assert_eq!(part_two(&input), 3263827);
    }
}
