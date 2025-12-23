//! Day 2: Gift Shop
//!
//! ## Problem Description
//!
//! Find invalid product IDs in given ranges. An ID is invalid if it's made of
//! a sequence of digits repeated.
//!
//! **Part 1**: Invalid if some sequence is repeated exactly twice.
//! Example: 11 (5 repeated), 22 (2 repeated), 1010 (10 repeated)
//!
//! **Part 2**: Invalid if some sequence is repeated at least twice.
//! Example: 111 (1 repeated 3x), 999 (9 repeated 3x), 565656 (56 repeated 3x)
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse comma-separated ranges "a-b" into (a, b) pairs.
//!
//! **Part 1 Strategy**: For each number in each range, check if it consists of
//! a substring repeated exactly twice. Sum all such numbers.
//! **Complexity**: O(n * d * L) where n is count of numbers, d is number of
//! divisors, L is length. Optimize by computing only once per number.
//!
//! **Part 2 Strategy**: Same but allow any repetition count >= 2.

/// Parse comma-separated ranges like "11-22,95-115"
fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

/// Check if a number consists of a substring repeated exactly twice.
/// e.g., "1212" -> true (12 repeated), "123123" -> true (123 repeated)
fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];
    first_half == second_half && !first_half.starts_with('0')
}

/// Check if a number consists of a substring repeated at least twice.
/// e.g., "1212" -> true, "123123" -> true, "111" -> true, "12341234" -> true
fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if len < 2 {
        return false;
    }
    // Check all possible divisor lengths (substring lengths)
    for div in 1..len {
        if len.is_multiple_of(div) {
            let repeats = len / div;
            if repeats >= 2 {
                let substr = &s[..div];
                // Verify the entire string is this substring repeated
                let mut valid = true;
                let mut expected_start = 0;
                for _ in 1..repeats {
                    let next_start = expected_start + div;
                    if &s[next_start..next_start + div] != substr {
                        valid = false;
                        break;
                    }
                    expected_start = next_start;
                }
                if valid && !substr.starts_with('0') {
                    return true;
                }
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for (start, end) in ranges {
        for n in start..=end {
            if is_repeated_twice(n) {
                sum += n;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let ranges = parse_input(input);
    let mut sum = 0;
    for (start, end) in ranges {
        for n in start..=end {
            if is_repeated_at_least_twice(n) {
                sum += n;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 1227775554);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(2);
        assert_eq!(part_two(&input), 4174379265);
    }
}
