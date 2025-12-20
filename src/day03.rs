//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Part 1: For each battery bank (line of digits), find the maximum
//! joltage by selecting exactly 2 batteries to form a 2-digit number.
//! Sum all maximum joltages.
//!
//! Part 2: Select exactly 12 batteries from each bank to form the
//! largest 12-digit number. Sum all maximum joltages.
//!
//! ## Solution Approach
//!
//! **Part 1**: For each bank, find the two largest digits. Concatenate
//! them in order of appearance to form the maximum 2-digit number.
//!
//! **Part 2**: For each bank, select the 12 largest digits while
//! preserving their order. This can be done by iterating through and
//! selecting the top 12 digits by value, maintaining position order.
//!
//! **Complexity**: O(n*m) where n is the number of banks and m is the
//! bank length.

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn max_joltage_n(bank: &str, n: usize) -> i64 {
    let chars: Vec<char> = bank.chars().collect();
    let total = chars.len();

    // Greedy approach: at each position in the result, pick the
    // largest digit from the remaining input that leaves enough
    // digits for the rest
    let mut result = String::new();
    let mut start_pos = 0;

    for digits_needed in (1..=n).rev() {
        // We need to leave (digits_needed - 1) more digits after this one
        let search_end = total - (digits_needed - 1);

        // Find the maximum digit in the valid range
        let mut max_digit = '0';
        let mut max_pos = start_pos;

        for i in start_pos..search_end {
            if chars[i] > max_digit {
                max_digit = chars[i];
                max_pos = i;
            }
        }

        result.push(max_digit);
        start_pos = max_pos + 1;
    }

    result.parse().unwrap()
}

pub fn part_one(input: &str) -> i64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage_n(bank, 2)).sum()
}

pub fn part_two(input: &str) -> i64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage_n(bank, 12)).sum()
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
