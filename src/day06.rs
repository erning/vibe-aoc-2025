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
//! - The operator position indicates which problem a number belongs to
//!
//! **Part 1 Strategy**:
//! - Parse each row into numbers with their column positions
//! - Scan operator row to find problem operators
//! - For each problem, assign numbers based on which operator position is closest
//! - Apply operator and sum results
//!
//! **Part 2 Strategy**:
//! - Cephalopod math reads character columns right-to-left
//! - For each character column, determine which problem it belongs to
//! - Read each character column top-to-bottom to form a number
//! - Group numbers by problem and apply operators
//!
//! **Complexity**: O(r * c) where r is rows and c is columns.
//!
//! **Optimization Note**: We scan the grid to identify which columns belong to
//! each problem, then read those columns to extract numbers.

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
    col_end: usize,
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
                col_end: col,
            });
        }
    }

    numbers
}

pub fn part_one(input: &str) -> u128 {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return 0;
    }

    let num_data_rows = grid.len() - 1;
    let op_row = &grid[grid.len() - 1];

    // Find operator positions
    let mut problem_ops: Vec<(usize, char)> = Vec::new();
    for (col, &ch) in op_row.iter().enumerate() {
        if ch == '+' || ch == '*' {
            problem_ops.push((col, ch));
        }
    }

    let problem_centers: Vec<usize> =
        problem_ops.iter().map(|(c, _)| *c).collect();

    // Parse all data rows into numbers
    let mut all_numbers: Vec<Vec<NumberAt>> = Vec::new();
    for row_idx in 0..num_data_rows {
        all_numbers.push(parse_row(&grid[row_idx]));
    }

    // Assign each number to the closest problem center
    let mut problem_numbers: Vec<Vec<u128>> =
        vec![Vec::new(); problem_ops.len()];

    for row_numbers in &all_numbers {
        for num in row_numbers {
            let num_center = (num.col_start + num.col_end) / 2;

            // Find closest problem center
            let mut closest_idx = 0;
            let mut closest_dist = usize::MAX;
            for (idx, &center) in problem_centers.iter().enumerate() {
                let dist = if center >= num_center {
                    center - num_center
                } else {
                    num_center - center
                };
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_idx = idx;
                }
            }

            problem_numbers[closest_idx].push(num.value);
        }
    }

    // Compute results
    let mut total: u128 = 0;
    for (idx, &(_, op)) in problem_ops.iter().enumerate() {
        let result: u128 = match op {
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
    let op_row = &grid[grid.len() - 1];

    // Find operator positions
    let mut problem_ops: Vec<(usize, char)> = Vec::new();
    for (col, &ch) in op_row.iter().enumerate() {
        if ch == '+' || ch == '*' {
            problem_ops.push((col, ch));
        }
    }

    let problem_centers: Vec<usize> =
        problem_ops.iter().map(|(c, _)| *c).collect();

    // Find max column width
    let max_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // For each character column, determine which problem it belongs to
    // Then read that column top-to-bottom to form a number
    let mut problem_numbers: Vec<Vec<u128>> =
        vec![Vec::new(); problem_ops.len()];

    for col in 0..max_cols {
        // Check if this column has any digits
        let mut has_digits = false;
        for row_idx in 0..num_data_rows {
            if col < grid[row_idx].len()
                && grid[row_idx][col].is_ascii_digit()
            {
                has_digits = true;
                break;
            }
        }

        if !has_digits {
            continue;
        }

        // Find which problem this column belongs to (closest operator)
        let mut closest_idx = 0;
        let mut closest_dist = usize::MAX;
        for (idx, &center) in problem_centers.iter().enumerate() {
            let dist = if center >= col {
                center - col
            } else {
                col - center
            };
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = idx;
            }
        }

        // Read this column top-to-bottom to form a number
        let mut num_str = String::new();
        for row_idx in 0..num_data_rows {
            if col < grid[row_idx].len()
                && grid[row_idx][col].is_ascii_digit()
            {
                num_str.push(grid[row_idx][col]);
            }
        }

        if !num_str.is_empty() {
            if let Ok(num) = num_str.parse::<u128>() {
                problem_numbers[closest_idx].push(num);
            }
        }
    }

    // Compute results - but we need to process numbers right-to-left within each problem
    let mut total: u128 = 0;
    for (idx, &(_, op)) in problem_ops.iter().enumerate() {
        // Sort by column position in descending order (right-to-left)
        // We need to track which number came from which column
        // But since we added them left-to-right, reverse the vector
        let numbers: Vec<u128> =
            problem_numbers[idx].iter().rev().copied().collect();

        let result: u128 = match op {
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
