//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Part 1: Find the largest number formed by selecting exactly 2 digits from each line
//!         while preserving the original order of digits.
//! Part 2: Find the largest number formed by selecting exactly 12 digits from each line
//!         while preserving the original order of digits.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line is a string of digits (1-9). Parse into a list of strings.
//!
//! **Part 1 Strategy**: For each line, find the largest 2-digit subsequence:
//! - To maximize a number with k digits, we want the largest possible leftmost digit
//! - Then the largest possible next digit from what remains, etc.
//! - This is a greedy approach: for position i (0-indexed), pick the largest digit
//!   that leaves enough digits remaining to fill positions i+1 to k-1
//! - For k=2: find the max digit in the first (n-1) positions, then find max digit
//!   in the remaining positions after that
//!
//! **Part 2 Strategy**: Same greedy approach but with k=12:
//! - For each of the 12 positions, pick the largest digit that allows completion
//! - At position i, we need at least (k - i - 1) digits remaining after our choice
//! - So we can only look at digits from position 0 to (n - (k - i)) inclusive
//!
//! **Complexity**: O(L * k * n) where L is number of lines, k is digits to select (2 or 12),
//! n is the line length. For each line and each position, we scan up to n digits.
//!
//! **Optimization Note**: Could use more sophisticated data structures, but the simple
//! greedy scan is fast enough for the given input sizes.

fn parse_input(input: &str) -> Vec<String> {
    input.trim().lines().map(|s| s.to_string()).collect()
}

fn max_subsequence(digits: &str, k: usize) -> u64 {
    let chars: Vec<char> = digits.chars().collect();
    let n = chars.len();
    let mut result = String::new();
    let mut start = 0;

    for i in 0..k {
        // We need to pick position i such that we leave enough for remaining (k - i - 1)
        // So we can look at positions from 'start' to (n - (k - i)) inclusive
        let remaining_needed = k - i - 1;
        let end = n - remaining_needed;

        // Find the maximum digit in the valid range
        let max_digit = chars[start..end].iter().max().unwrap();

        // Find its position and add it to result
        let pos = chars[start..end]
            .iter()
            .position(|c| c == max_digit)
            .unwrap();
        result.push(*max_digit);
        start = start + pos + 1;
    }

    result.parse().unwrap()
}

pub fn part_one(input: &str) -> u64 {
    let lines = parse_input(input);
    lines.iter().map(|line| max_subsequence(line, 2)).sum()
}

pub fn part_two(input: &str) -> u64 {
    let lines = parse_input(input);
    lines.iter().map(|line| max_subsequence(line, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 357);
        assert_eq!(part_two(&input), 3121910778619);
    }
}
