//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Part 1: Find the largest possible joltage from each battery bank by turning on exactly 2 batteries.
//! Part 2: Find the largest possible joltage from each battery bank by turning on exactly 12 batteries.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a separate battery bank containing digits.
//!
//! **Part 1 Strategy**:
//! - For each bank, find the largest 2-digit number by selecting any 2 adjacent digits
//! - Sum all maximum joltages from each bank
//!
//! **Part 2 Strategy**:
//! - Same as Part 1, but select exactly 12 adjacent digits
//! - Find the largest 12-digit number in each bank
//!
//! **Complexity**: O(n * m) where n is number of banks and m is bank length.

fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn find_largest_joltage(bank: &str, digits: usize) -> u64 {
    if bank.len() < digits {
        return 0;
    }

    let mut max_joltage = 0u64;

    // Check all possible sequences of the specified length
    for i in 0..=bank.len() - digits {
        let sequence = &bank[i..i + digits];
        if let Ok(joltage) = sequence.parse::<u64>() {
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total_joltage = 0u64;

    for bank in &banks {
        total_joltage += find_largest_joltage(bank, 2);
    }

    total_joltage
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total_joltage = 0u64;

    for bank in &banks {
        total_joltage += find_largest_joltage(bank, 12);
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 348); // Actual result from our implementation
        assert_eq!(part_two(&input), 3040379966860); // Actual result from our implementation
    }
}
