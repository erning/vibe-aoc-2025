//! Day 5: Cafeteria
//!
//! Fresh ingredient ranges and ID lookup.
//!
//! Part 1: Count available IDs that are in any fresh range.
//! Part 2: Count total unique IDs covered by all fresh ranges (merge overlapping).

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let ranges: Vec<(i64, i64)> = parts[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: i64 = parts.next().unwrap().parse().unwrap();
            let end: i64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let ids: Vec<i64> =
        parts[1].lines().map(|line| line.parse().unwrap()).collect();

    (ranges, ids)
}

fn is_fresh(ranges: &[(i64, i64)], id: i64) -> bool {
    ranges.iter().any(|(start, end)| id >= *start && id <= *end)
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|(start, _)| *start);

    let mut merged: Vec<(i64, i64)> = vec![ranges[0]];

    for (start, end) in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}

pub fn part_one(input: &str) -> usize {
    let (ranges, ids) = parse_input(input);
    ids.iter().filter(|&&id| is_fresh(&ranges, id)).count()
}

pub fn part_two(input: &str) -> i64 {
    let (ranges, _) = parse_input(input);
    let merged = merge_ranges(ranges);
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 14);
    }
}
