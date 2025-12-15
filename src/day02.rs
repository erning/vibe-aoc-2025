//! Day 2: Gift Shop
//!
//! Find invalid product IDs that are digit sequences repeated.
//!
//! Part 1: IDs where some digit sequence is repeated exactly twice.
//! Part 2: IDs where some digit sequence is repeated at least twice.

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let half = len / 2;
    s[..half] == s[half..]
}

fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    // Try all divisors of len that are >= 2
    for repeat_count in 2..=len {
        if len.is_multiple_of(repeat_count) {
            let unit_len = len / repeat_count;
            let unit = &s[..unit_len];
            if s.chars()
                .collect::<Vec<_>>()
                .chunks(unit_len)
                .all(|chunk| chunk.iter().collect::<String>() == unit)
            {
                return true;
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
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 1227775554);
        assert_eq!(part_two(&input), 4174379265);
    }
}
