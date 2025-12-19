//! Day 03: Lobby
//!
//! ## Problem Description
//!
//! You need to power an escalator by turning on batteries. The batteries are arranged in banks (lines of digits),
//! and each bank requires turning on exactly two batteries for Part 1 (or exactly twelve batteries for Part 2).
//! The joltage output is the number formed by the digits of the batteries you turn on.
//!
//! **Part 1**: Find the largest possible joltage each bank can produce by turning on exactly two batteries.
//! **Part 2**: Find the largest possible joltage each bank can produce by turning on exactly twelve batteries.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a bank of batteries (string of digits).
//! For each bank: Find the maximum number possible by selecting the specified number of digits in order.
//! Use a greedy algorithm: for each selection, pick the largest possible digit that allows enough remaining digits.
//!
//! **Complexity**: O(N*M) where N is number of banks and M is average length of each bank.
//!
//! ## Algorithm Explanation
//!
//! For each part, we use a greedy approach:
//! - For each digit position in the result, scan the input to find the largest digit that can be placed
//!   while leaving enough digits to fill the remaining positions.

fn parse_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total_joltage = 0;

    for bank in banks {
        // For each bank, we need to turn on exactly 2 batteries to form the largest possible number
        let max_joltage = find_max_joltage(bank, 2);
        total_joltage += max_joltage;
    }

    total_joltage
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total_joltage = 0;

    for bank in banks {
        // For each bank, we need to turn on exactly 12 batteries to form the largest possible number
        let max_joltage = find_max_joltage(bank, 12);
        total_joltage += max_joltage;
    }

    total_joltage
}

fn find_max_joltage(bank: &str, num_batteries: usize) -> u64 {
    if num_batteries >= bank.len() {
        // If we need to turn on more or equal batteries than available,
        // just take the whole string
        return bank.parse().unwrap_or(0);
    }

    let bank_chars: Vec<char> = bank.chars().collect();
    let mut result = Vec::new();
    let mut start_idx = 0;

    // For each of the required batteries/digits
    for _ in 0..num_batteries {
        // We need to find the largest digit in the remaining range that allows
        // us to have enough digits left to complete the number
        let remaining_to_select = num_batteries - result.len();
        // We can select from start_idx up to (bank_chars.len() - remaining_to_select)
        // because we need to leave remaining_to_select digits for the future selections
        let search_end = bank_chars.len() - remaining_to_select;

        // Find the largest digit and its position in the valid range [start_idx, search_end]
        let mut max_digit = '0';
        let mut max_pos = start_idx;

        for (idx, &digit) in
            bank_chars[start_idx..=search_end].iter().enumerate()
        {
            let actual_idx = start_idx + idx;
            if digit > max_digit {
                max_digit = digit;
                max_pos = actual_idx;
            }
        }

        // Add the best digit to our result
        result.push(max_digit);
        // Move start index to the position after the one we selected
        start_idx = max_pos + 1;
    }

    let result_str: String = result.iter().collect();
    result_str.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 357); // 98 + 89 + 78 + 92
    }

    #[test]
    fn example_part_two() {
        let input = read_example(3);
        assert_eq!(part_two(&input), 3121910778619); // 987654321111 + 811111111119 + 434234234278 + 888911112111
    }
}
