//! Day 6: Trash Compactor
//!
//! Cephalopod math worksheet with vertical problems.
//!
//! Part 1: Parse numbers normally (left to right), apply operations, sum results.
//! Part 2: Read numbers column by column right-to-left, apply operations, sum results.

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part1(grid: &[Vec<char>]) -> u64 {
    let height = grid.len();
    let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    let get_char = |row: usize, col: usize| -> char {
        if col < grid[row].len() {
            grid[row][col]
        } else {
            ' '
        }
    };

    let mut col = 0;
    let mut total: u64 = 0;

    while col < width {
        // Skip space columns
        while col < width && (0..height).all(|row| get_char(row, col) == ' ')
        {
            col += 1;
        }
        if col >= width {
            break;
        }

        // Find end of this problem (next all-space column or end)
        let start_col = col;
        while col < width && !(0..height).all(|row| get_char(row, col) == ' ')
        {
            col += 1;
        }
        let end_col = col;

        // Parse numbers and operator
        let mut numbers: Vec<u64> = Vec::new();
        let mut operator = '+';

        for row in 0..height {
            let mut num_str = String::new();
            for c in start_col..end_col {
                let ch = get_char(row, c);
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                } else if ch == '+' || ch == '*' {
                    operator = ch;
                }
            }
            if !num_str.is_empty() {
                numbers.push(num_str.parse().unwrap());
            }
        }

        let result = match operator {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };
        total += result;
    }

    total
}

fn solve_part2(grid: &[Vec<char>]) -> u64 {
    let height = grid.len();
    let width = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    let get_char = |row: usize, col: usize| -> char {
        if col < grid[row].len() {
            grid[row][col]
        } else {
            ' '
        }
    };

    // Process from right to left
    let mut col = width;
    let mut total: u64 = 0;

    while col > 0 {
        col -= 1;

        // Skip space columns
        while col > 0 && (0..height).all(|row| get_char(row, col) == ' ') {
            col -= 1;
        }

        // Find start of this problem (next all-space column or beginning)
        let end_col = col + 1;
        while col > 0 && !(0..height).all(|row| get_char(row, col - 1) == ' ')
        {
            col -= 1;
        }
        let start_col = col;

        if start_col >= end_col {
            break;
        }

        // Parse numbers column by column (right to left) and operator
        // Each column with digits forms ONE number (digits top-to-bottom = most-to-least significant)
        let mut numbers: Vec<u64> = Vec::new();
        let mut operator = '+';

        // Read columns from right to left
        for c in (start_col..end_col).rev() {
            // Collect digits in this column (top to bottom)
            let mut num_str = String::new();
            for row in 0..height - 1 {
                let ch = get_char(row, c);
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                }
            }
            if !num_str.is_empty() {
                numbers.push(num_str.parse().unwrap());
            }

            let op_ch = get_char(height - 1, c);
            if op_ch == '+' || op_ch == '*' {
                operator = op_ch;
            }
        }

        let result: u64 = match operator {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        };
        total += result;
    }

    total
}

pub fn part_one(input: &str) -> u64 {
    let grid = parse_input(input);
    solve_part1(&grid)
}

pub fn part_two(input: &str) -> u64 {
    let grid = parse_input(input);
    solve_part2(&grid)
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
