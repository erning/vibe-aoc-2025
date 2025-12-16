//! Day 3: Lobby
//!
//! Select batteries from banks to maximize joltage. The joltage is the number
//! formed by the selected digit positions in their original order.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Each line is a bank of battery joltage ratings (digits).
//!
//! **Part 1 Strategy**: Select exactly 2 batteries to maximize joltage.
//! **Part 2 Strategy**: Select exactly 12 batteries to maximize joltage.
//!
//! **Greedy Algorithm**: To select k digits forming the maximum number:
//! - Process positions left to right
//! - At each step, from the current position to (n - remaining_picks + 1),
//!   find the position with the maximum digit
//! - Select that digit and move past it
//! - Repeat until k digits are selected
//!
//! This greedy approach works because:
//! - Higher-order digit positions dominate lower ones
//! - We must ensure enough digits remain for remaining selections
//! - Complexity: O(n * k) per line

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

/// Select k digits from the bank to form the maximum possible number.
/// Returns the value as u64.
fn max_joltage(bank: &[u8], k: usize) -> u64 {
    let n = bank.len();
    if k == 0 || k > n {
        return 0;
    }

    let mut result: u64 = 0;
    let mut start = 0;
    let mut remaining = k;

    while remaining > 0 {
        // We need to pick one digit now, and leave room for (remaining-1) more
        // So we can search from `start` to `n - remaining` (inclusive)
        let end = n - remaining;

        // Find the position of the maximum digit in [start, end]
        // Use fold to get the first (leftmost) max, preserving flexibility
        let (best_pos, best_val) = bank[start..=end].iter().enumerate().fold(
            (0, bank[start]),
            |(best_i, best_v), (i, &v)| {
                if v > best_v {
                    (i, v)
                } else {
                    (best_i, best_v)
                }
            },
        );
        let best_pos = start + best_pos;

        // Append this digit to our result
        result = result * 10 + best_val as u64;

        // Move past the selected position
        start = best_pos + 1;
        remaining -= 1;
    }

    result
}

pub fn part_one(input: &str) -> u64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank, 2)).sum()
}

pub fn part_two(input: &str) -> u64 {
    let banks = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank, 12)).sum()
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

    #[test]
    fn test_max_joltage() {
        // 987654321111111 -> select 2 -> "98" (positions 0,1)
        let bank1: Vec<u8> =
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(max_joltage(&bank1, 2), 98);

        // 811111111111119 -> select 2 -> "89" (positions 0,14)
        let bank2: Vec<u8> =
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(max_joltage(&bank2, 2), 89);

        // 234234234234278 -> select 2 -> "78" (positions 13,14)
        let bank3: Vec<u8> =
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(max_joltage(&bank3, 2), 78);

        // 818181911112111 -> select 2 -> "92" (positions 6,11)
        let bank4: Vec<u8> =
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(max_joltage(&bank4, 2), 92);
    }
}
