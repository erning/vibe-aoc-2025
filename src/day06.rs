//! Day 6: Trash Compactor
//!
//! Help a cephalopod with their math homework by parsing and solving
//! vertically-arranged arithmetic problems.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse a grid where each column belongs to a problem.
//! Problems are separated by a column of spaces. The last row contains
//! the operator (+ or *).
//!
//! **Part 1 Strategy**: Read numbers left-to-right, top-to-bottom.
//! - Each number appears in a column, padded with spaces
//! - Parse each column's digits as a number
//! - Apply operator and sum results
//! - Complexity: O(rows * cols)
//!
//! **Part 2 Strategy**: Read columns right-to-left within each problem.
//! - Each number is read column-by-column right-to-left
//! - Most significant digit at top, least at bottom
//! - Complexity: O(rows * cols)

/// A parsed problem with numbers and an operator
#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => panic!("Unknown operator: {}", self.operator),
        }
    }
}

/// Parse the input grid into a 2D character array
fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // Find the maximum line width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Build a grid, padding each line to max width with spaces
    lines
        .iter()
        .map(|line| {
            let mut row: Vec<char> = line.chars().collect();
            row.resize(max_width, ' ');
            row
        })
        .collect()
}

/// Identify problem boundaries (columns that are all spaces except last row)
fn find_problem_boundaries(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let height = grid.len();
    let width = grid[0].len();
    let data_rows = height - 1; // Last row is operators

    // Find separator columns (all spaces in data rows)
    let mut is_separator: Vec<bool> = vec![true; width];
    for row in grid.iter().take(data_rows) {
        for (col, sep) in is_separator.iter_mut().enumerate() {
            if row[col] != ' ' {
                *sep = false;
            }
        }
    }

    // Find contiguous problem regions
    let mut boundaries = vec![];
    let mut start: Option<usize> = None;

    for (col, &sep) in is_separator.iter().enumerate() {
        if sep {
            if let Some(s) = start {
                boundaries.push((s, col));
                start = None;
            }
        } else if start.is_none() {
            start = Some(col);
        }
    }

    // Handle last problem if not terminated by separator
    if let Some(s) = start {
        boundaries.push((s, width));
    }

    boundaries
}

/// Parse problems for Part 1 (left-to-right number reading)
fn parse_problems_part1(input: &str) -> Vec<Problem> {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return vec![];
    }

    let height = grid.len();
    let boundaries = find_problem_boundaries(&grid);

    boundaries
        .iter()
        .map(|&(start, end)| {
            // Get the operator from the last row
            let operator = grid[height - 1][start..end]
                .iter()
                .find(|&&c| c == '+' || c == '*')
                .copied()
                .unwrap_or('+');

            // Parse each row's number (read left-to-right within problem)
            let numbers: Vec<u64> = (0..height - 1)
                .filter_map(|row| {
                    let num_str: String = grid[row][start..end]
                        .iter()
                        .filter(|c| c.is_ascii_digit())
                        .collect();
                    if num_str.is_empty() {
                        None
                    } else {
                        Some(num_str.parse().unwrap())
                    }
                })
                .collect();

            Problem { numbers, operator }
        })
        .collect()
}

/// Parse problems for Part 2 (each column is a separate number, read right-to-left)
fn parse_problems_part2(input: &str) -> Vec<Problem> {
    let grid = parse_grid(input);
    if grid.is_empty() {
        return vec![];
    }

    let height = grid.len();
    let boundaries = find_problem_boundaries(&grid);

    boundaries
        .iter()
        .map(|&(start, end)| {
            // Get the operator from the last row
            let operator = grid[height - 1][start..end]
                .iter()
                .find(|&&c| c == '+' || c == '*')
                .copied()
                .unwrap_or('+');

            // For Part 2, each COLUMN becomes a separate number
            // Most significant digit at top, least at bottom
            // We read columns right-to-left
            let data_rows = height - 1;

            // Collect numbers from columns, reading right-to-left
            let numbers: Vec<u64> = (start..end)
                .rev()
                .filter_map(|col| {
                    // Build a number from this column (top to bottom)
                    let digits: String = (0..data_rows)
                        .filter_map(|row| {
                            let c = grid[row][col];
                            if c.is_ascii_digit() {
                                Some(c)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if digits.is_empty() {
                        None
                    } else {
                        Some(digits.parse().unwrap())
                    }
                })
                .collect();

            Problem { numbers, operator }
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let problems = parse_problems_part1(input);
    problems.iter().map(|p| p.solve()).sum()
}

pub fn part_two(input: &str) -> u64 {
    let problems = parse_problems_part2(input);
    problems.iter().map(|p| p.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part1() {
        let input = read_example(6);
        // 123*45*6 = 33210, 328+64+98 = 490, 51*387*215 = 4243455, 64+23+314 = 401
        // Grand total = 33210 + 490 + 4243455 + 401 = 4277556
        assert_eq!(part_one(&input), 4277556);
    }

    #[test]
    fn example_part2() {
        let input = read_example(6);
        // Right-to-left: 356*24*1 = 8544, 8+248+369 = 625,
        // 175*581*32 = 3253600, 4+431+623 = 1058
        // Grand total = 8544 + 625 + 3253600 + 1058 = 3263827
        assert_eq!(part_two(&input), 3263827);
    }
}
