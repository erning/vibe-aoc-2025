//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Part 1: From each line of digits (battery bank), find the largest possible 2-digit
//! number by selecting any two digits in order. Sum all these maximum values.
//!
//! Part 2: From each line of digits, find the largest possible 12-digit
//! number by selecting exactly twelve digits in order. Sum all these maximum values.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Read each line as a sequence of digits.
//!
//! **Part 1**: For each line, find the two largest digits that appear in order.
//! The maximum 2-digit number is formed by the leftmost largest digit followed
//! by the rightmost largest digit that comes after it.
//!
//! **Part 2**: For each line, we need exactly 12 digits. We use a greedy approach:
//! - If the line has exactly 12 digits, take them all
//! - If more than 12, we need to drop some digits. We prioritize dropping
//!   smaller digits to maximize the final number.
//! - If less than 12, we cannot form a 12-digit number (but based on puzzle,
//!   this case doesn't occur)
//!
//! ## Complexity Analysis
//!
//! Let N be the number of lines and L be the average line length.
//! - Time Complexity: O(N * L) for both parts
//! - Space Complexity: O(1) - constant extra space

/// Parse the input into a vector of digit strings
pub fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

/// Find the maximum 2-digit number from a digit sequence
fn max_two_digit_number(digits: &str) -> u64 {
    let chars: Vec<char> = digits.chars().collect();
    let mut max_val = 0;

    // Find the best pair maintaining order
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            if let (Some(d1), Some(d2)) = (chars[i].to_digit(10), chars[j].to_digit(10)) {
                let val = (d1 * 10 + d2) as u64;
                max_val = max_val.max(val);
            }
        }
    }

    max_val
}

/// Find the maximum 12-digit number from a digit sequence
fn max_twelve_digit_number(digits: &str) -> u64 {
    let chars: Vec<char> = digits.chars().collect();

    // For Part 2, we need exactly 12 digits selected in order
    // Use dynamic programming to find the maximum number

    if chars.len() < 12 {
        return 0;
    }

    // We'll use a greedy approach - always try to pick larger digits when possible
    let mut result = String::new();
    let mut pos = 0;

    // We need to pick 12 digits
    for _ in 0..12 {
        // Find the largest digit we can pick
        let mut best_digit = '0';
        let mut best_pos = pos;

        // Search from current position to the end
        // But we need to leave enough digits for the remaining picks
        let max_search_pos = chars.len() - (12 - result.len());

        for i in pos..=max_search_pos {
            if chars[i] > best_digit {
                best_digit = chars[i];
                best_pos = i;
                if best_digit == '9' {
                    break; // Can't do better than 9
                }
            }
        }

        result.push(best_digit);
        pos = best_pos + 1;
    }

    result.parse().unwrap_or(0)
}

/// Part 1: Sum of maximum 2-digit numbers from each line
pub fn part_one(input: &str) -> impl std::fmt::Display {
    let lines = parse_input(input);
    let mut total = 0u64;

    for line in lines {
        total += max_two_digit_number(&line);
    }

    total
}

/// Part 2: Sum of maximum 12-digit numbers from each line
pub fn part_two(input: &str) -> impl std::fmt::Display {
    let lines = parse_input(input);
    let mut total = 0u64;

    for line in lines {
        total += max_twelve_digit_number(&line);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_two_digit_number() {
        assert_eq!(max_two_digit_number("987654321111111"), 98);
        assert_eq!(max_two_digit_number("811111111111119"), 89);
        assert_eq!(max_two_digit_number("234234234234278"), 78);
        assert_eq!(max_two_digit_number("81818191112111211"), 92);
    }

    #[test]
    fn test_max_twelve_digit_number() {
        assert_eq!(max_twelve_digit_number("987654321111111"), 987654321111);
        assert_eq!(max_twelve_digit_number("811111111111119"), 811111111119);
        assert_eq!(max_twelve_digit_number("234234234234278"), 434234234278);
        assert_eq!(max_twelve_digit_number("81818191112111211"), 888911112111);
    }

    #[test]
    fn example() {
        let input = "987654321111111
811111111111119
234234234234278
81818191112111211";

        assert_eq!(part_one(input).to_string(), "357");
        assert_eq!(part_two(input).to_string(), "3121910778619");
    }
}