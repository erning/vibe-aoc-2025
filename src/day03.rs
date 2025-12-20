//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Part 1: For each line (bank) of digit characters, select exactly 2
//! batteries (digit positions) and form the largest 2-digit number possible
//! from those digits (in order). Sum the results for all banks.
//!
//! Part 2: Similar to Part 1, but select exactly 12 batteries instead of 2,
//! forming the largest 12-digit number possible. Sum the results for all
//! banks.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line is a bank of batteries represented as a
//! string of digit characters.
//!
//! **Part 1 Strategy**:
//! - For each bank, find the indices of the two largest digits
//! - Select the largest digit, then select the largest digit after it
//! - Form a 2-digit number from these digits in their original order
//! - Greedy approach: prefer rightmost largest digits to avoid blocking
//!   future selections
//!
//! **Part 2 Strategy**:
//! - Similar to Part 1, but select 12 batteries instead of 2
//! - Use greedy approach: select the largest available digit at each step
//!
//! **Complexity**: O(n * m * k) where n is number of banks, m is max
//! length per bank, k is number of batteries to select (2 or 12).

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn select_largest_joltage(bank: &str, count: usize) -> u64 {
    let digits: Vec<u32> =
        bank.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let len = digits.len();
    let actual_count = count.min(len);

    // Use lookahead greedy approach: at each step, look ahead to find the
    // best digit we can select while still having enough positions left to
    // select the remaining digits needed.
    let mut selected_indices = Vec::new();
    let mut current_pos = 0;

    for _ in 0..actual_count {
        let remaining_needed = actual_count - selected_indices.len();
        let available_after_current = len - current_pos;

        // How far ahead can we look without running out of positions?
        // We can skip up to (available - remaining_needed) positions
        let max_skip = available_after_current - remaining_needed;

        // Find the maximum digit in the range [current_pos, current_pos +
        // max_skip], preferring earlier indices on ties
        let search_slice = &digits[current_pos..=(current_pos + max_skip)];
        let mut max_offset = 0;
        let mut max_val = search_slice[0];

        for (offset, &val) in search_slice.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_offset = offset;
            }
        }

        let best_idx = current_pos + max_offset;

        selected_indices.push(best_idx);
        current_pos = best_idx + 1;
    }

    // Build the number from selected digits in order
    let mut result = 0u64;
    for idx in selected_indices {
        result = result * 10 + (digits[idx] as u64);
    }

    result
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total = 0;

    for bank in banks {
        total += select_largest_joltage(bank, 2);
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut total = 0;

    for bank in banks {
        total += select_largest_joltage(bank, 12);
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
