//! Day 3: Lobby
//!
//! ## Puzzle Overview
//!
//! The Elves need to power up an escalator by finding the maximum joltage from each
//! bank of batteries. Each bank is a line of digits (batteries), and we need to
//! turn on exactly k batteries (k=2 for Part 1, k=12 for Part 2) to form the
//! largest possible joltage number.
//!
//! ## Part 1 Strategy
//!
//! For each bank (line of digits), we need to find the maximum joltage that can
//! be formed by selecting exactly 2 digits in their original order. The joltage
//! is formed by concatenating the selected digits.
//!
//! **Algorithm**:
//! 1. Parse input into vector of strings (banks)
//! 2. For each bank, find all possible 2-digit combinations
//! 3. Convert each combination to a number and find the maximum
//! 4. Sum all maximum values
//!
//! **Complexity**: O(n * k^2 * d) where n is number of banks, k is digits to select,
//! and d is average bank length
//!
//! ## Part 2 Strategy
//!
//! Same as Part 1 but selecting exactly 12 digits instead of 2.
//!
//! **Algorithm**:
//! 1. Parse input into vector of strings (banks)
//! 2. For each bank, find all possible 12-digit combinations
//! 3. Convert each combination to a number and find the maximum
//! 4. Sum all maximum values
//!
//! **Complexity**: O(n * C(d,k) * k) where n is number of banks, d is bank length,
//! and C(d,k) is the number of combinations

fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

/// Find maximum joltage from a bank by selecting exactly k digits
/// Uses a greedy algorithm: at each step, select the largest possible digit
/// that still allows selecting the remaining k-1 digits
fn max_joltage(bank: &str, k: usize) -> u64 {
    let digits: Vec<char> = bank.chars().collect();
    let len = digits.len();

    if k >= len {
        return bank.parse().unwrap();
    }

    let mut result = String::new();
    let mut start_idx = 0;

    for i in 0..k {
        let remaining = k - i - 1;
        let end_idx = len - remaining;

        // Find the maximum digit in the range [start_idx, end_idx)
        let mut max_digit_idx = start_idx;
        let mut max_digit = digits[start_idx];

        for j in (start_idx + 1)..end_idx {
            if digits[j] > max_digit {
                max_digit = digits[j];
                max_digit_idx = j;
            }
        }

        result.push(max_digit);
        start_idx = max_digit_idx + 1;
    }

    result.parse().unwrap()
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total = 0u64;

    for bank in banks {
        total += max_joltage(&bank, 2);
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total = 0u64;

    for bank in banks {
        total += max_joltage(&bank, 12);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 357);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(3);
        assert_eq!(part_two(&input), 3121910778619);
    }
}
