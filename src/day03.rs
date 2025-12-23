//! Day 3: Lobby
//!
//! ## Problem Description
//!
//! Each line is a bank of batteries (digits 1-9). Turn on exactly k batteries
//! (k=2 for Part 1, k=12 for Part 2) to form a k-digit number. The digits must
//! be selected in their original order. Find the maximum possible number from
//! each bank and sum them.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line is a string of digits representing a bank.
//!
//! **Part 1 & 2 Strategy**: For each bank, select k digits in order to maximize
//! the resulting number. Algorithm:
//! - Find max digit among valid candidates
//! - Try all positions with that digit recursively
//! - Branching factor is small because we only try max-digit positions

/// Parse input into a vector of strings (each line is a bank)
fn parse_input(input: &str) -> Vec<String> {
    input.trim().lines().map(|s| s.trim().to_string()).collect()
}

/// Count unused characters from index i (exclusive) to end
fn count_available(used: &[bool], start: usize) -> usize {
    used[start..].iter().filter(|&&u| !u).count()
}

/// Recursively find the maximum k-digit number
fn find_max(
    chars: &[char],
    start_idx: usize,
    k: usize,
    used: &mut [bool],
) -> String {
    if k == 0 {
        return String::new();
    }

    let n = chars.len();

    // Find maximum digit among valid candidates
    let mut max_digit = '0';
    for i in start_idx..n {
        if used[i] {
            continue;
        }
        let available = count_available(used, i + 1);
        if available >= k - 1 && chars[i] > max_digit {
            max_digit = chars[i];
        }
    }

    // Try all positions with max_digit
    let mut best = String::new();
    for i in start_idx..n {
        if used[i] || chars[i] != max_digit {
            continue;
        }

        let available = count_available(used, i + 1);
        if available < k - 1 {
            continue;
        }

        used[i] = true;
        let suffix = find_max(chars, i + 1, k - 1, used);
        used[i] = false;

        let current = format!("{}{}", chars[i], suffix);
        if best.is_empty() || current > best {
            best = current;
        }
    }

    best
}

/// Find maximum k-digit number from s by selecting k digits in order.
fn max_number(s: &str, k: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if k == 0 || n < k {
        return String::new();
    }

    let mut used = vec![false; n];
    find_max(&chars, 0, k, &mut used)
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut sum = 0;
    for bank in banks {
        let max_num = max_number(&bank, 2);
        if !max_num.is_empty() {
            sum += max_num.parse::<u64>().unwrap();
        }
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut sum = 0;
    for bank in banks {
        let max_num = max_number(&bank, 12);
        if !max_num.is_empty() {
            sum += max_num.parse::<u64>().unwrap();
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
        let input = read_example(3);
        assert_eq!(part_one(&input), 357);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(3);
        assert_eq!(part_two(&input), 3121910778619);
    }
}
