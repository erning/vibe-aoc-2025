//! Day 3: Lobby
//!
//! Select digits from battery banks to form largest numbers.
//!
//! Part 1: Select exactly 2 batteries to form largest 2-digit number.
//! Part 2: Select exactly 12 batteries to form largest 12-digit number.

fn max_joltage(bank: &str, count: usize) -> u64 {
    let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
    let n = digits.len();

    // Greedy approach: for each position, pick the largest digit possible
    // while ensuring we can still fill remaining positions
    let mut result: u64 = 0;
    let mut pos = 0;
    for i in 0..count {
        let remaining = count - i - 1;
        // Find the largest digit we can pick such that there are enough digits left
        let max_start = n - remaining - 1;
        let mut best_digit = 0;
        let mut best_pos = pos;
        for (j, &digit) in
            digits.iter().enumerate().take(max_start + 1).skip(pos)
        {
            if digit > best_digit {
                best_digit = digit;
                best_pos = j;
            }
        }
        result = result * 10 + best_digit as u64;
        pos = best_pos + 1;
    }

    result
}

pub fn part_one(input: &str) -> u64 {
    input.trim().lines().map(|line| max_joltage(line, 2)).sum()
}

pub fn part_two(input: &str) -> u64 {
    input.trim().lines().map(|line| max_joltage(line, 12)).sum()
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
