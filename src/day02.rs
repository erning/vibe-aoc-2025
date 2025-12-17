//! # Day 2: Gift Shop
//!
//! ## Problem Description
//! You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.
//!
//! As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.
//!
//! As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?
//!
//! They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:
//!
//! ```
//! 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
//! 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
//! 824824821-824824827,2121212118-2121212124
//! ```
//! (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)
//!
//! The ranges are separated by commas (`,`); each range gives its _first ID_ and _last ID_ separated by a dash (`-`).
//!
//! Since the young Elf was just doing silly patterns, you can find the _invalid IDs_ by looking for any ID which is made only of some sequence of digits repeated twice. So, `55` (`5` twice), `6464` (`64` twice), and `123123` (`123` twice) would all be invalid IDs.
//!
//! None of the numbers have leading zeroes; `0101` isn't an ID at all. (`101` is a _valid_ ID that you would ignore.)
//!
//! Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:
//!
//! - `11-22` has two invalid IDs, `_11_` and `_22_`.
//! - `95-115` has one invalid ID, `_99_`.
//! - `998-1012` has one invalid ID, `_1010_`.
//! - `1188511880-1188511890` has one invalid ID, `_1188511885_`.
//! - `222220-222224` has one invalid ID, `_222222_`.
//! - `1698522-1698528` contains no invalid IDs.
//! - `446443-446449` has one invalid ID, `_446446_`.
//! - `38593856-38593862` has one invalid ID, `_38593859_`.
//! - The rest of the ranges contain no invalid IDs.
//!
//! Adding up all the invalid IDs in this example produces `_1227775554_`.
//!
//! _What do you get if you add up all of the invalid IDs?_
//!
//! ## Part Two
//! The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?
//!
//! Now, an ID is invalid if it is made only of some sequence of digits repeated _at least_ twice. So, `12341234` (`1234` two times), `123123123` (`123` three times), `1212121212` (`12` five times), and `1111111` (`1` seven times) are all invalid IDs.
//!
//! From the same example as before:
//!
//! - `11-22` still has two invalid IDs, `_11_` and `_22_`.
//! - `95-115` now has two invalid IDs, `_99_` and `_111_`.
//! - `998-1012` now has two invalid IDs, `_999_` and `_1010_`.
//! - `1188511880-1188511890` still has one invalid ID, `_1188511885_`.
//! - `222220-222224` still has one invalid ID, `_222222_`.
//! - `1698522-1698528` still contains no invalid IDs.
//! - `446443-446449` still has one invalid ID, `_446446_`.
//! - `38593856-38593862` still has one invalid ID, `_38593859_`.
//! - `565653-565659` now has one invalid ID, `_565656_`.
//! - `824824821-824824827` now has one invalid ID, `_824824824_`.
//! - `2121212118-2121212124` now has one invalid ID, `_2121212121_`.
//!
//! Adding up all the invalid IDs in this example produces `_4174379265_`.
//!
//! _What do you get if you add up all of the invalid IDs using these new rules?_
//!
//! ## Solution Approach
//! ### Part One
//! The problem asks us to identify "invalid IDs" within given ranges. An ID is invalid if its string representation consists of a sequence of digits repeated exactly twice. For example, `55` (5 repeated twice), `6464` (64 repeated twice), and `123123` (123 repeated twice). Leading zeros are not allowed, meaning `0101` is not considered an ID.
//!
//! The input is a single line containing comma-separated ranges (e.g., `11-22,95-115`). Each range specifies a `first ID` and `last ID`.
//!
//! **Overall Strategy:**
//! 1. Parse the input string into a list of `(start, end)` ID ranges.
//! 2. For each number within each range, check if it's an invalid ID.
//! 3. Sum all identified invalid IDs.
//!
//! **`parse_input` Function:**
//! - Takes a string input.
//! - Splits the input by `,`.
//! - For each segment, splits by `-` to get `start` and `end` values.
//! - Parses these into `u64` and returns a `Vec<(u64, u64)>`.
//!
//! **`is_invalid_part1(id: u64)` Function:**
//! - Converts the `id` to its string representation.
//! - Checks if the string length is even. If odd, it cannot be a sequence repeated twice, so it's a valid ID (returns `false`).
//! - Divides the string into two halves.
//! - Compares the two halves. If they are equal, the ID is invalid (returns `true`).
//! - Handles the "no leading zeros" rule implicitly since we convert `u64` to string, so `05` would become `5`.
//!
//! **`part_one` Function:**
//! - Calls `parse_input` to get the list of ranges.
//! - Initializes `total_invalid_ids = 0`.
//! - Iterates through each `(start, end)` range.
//! - For each `id` from `start` to `end` (inclusive):
//!   - Calls `is_invalid_part1(id)`.
//!   - If `true`, adds `id` to `total_invalid_ids`.
//! - Returns `total_invalid_ids.to_string()`.
//!
//! ### Part Two
//! Part Two updates the definition of an invalid ID: it's any ID made only of some sequence of digits repeated _at least_ twice. This means `12341234`, `123123123`, `1212121212`, `1111111` are all invalid.
//!
//! **`is_invalid_part2(id: u64)` Function:**
//! - Converts the `id` to its string representation `s`.
//! - Iterates through possible substring lengths `len` from `1` up to `s.len() / 2`.
//! - For each `len`:
//!   - If `s.len()` is not divisible by `len`, this `len` cannot be a repeating unit, so skip.
//!   - Extracts the `prefix = s[0..len]`.
//!   - Constructs a repeated string `repeated_s = prefix.repeat(s.len() / len)`.
//!   - If `s` equals `repeated_s`, then the ID is invalid (returns `true`).
//! - If no such repeating sequence is found after checking all possible `len`s, the ID is valid (returns `false`).
//!
//! **`part_two` Function:**
//! - Similar structure to `part_one`, but calls `is_invalid_part2(id)`.
//!
//! ### Example Walkthrough for Part 2 (`95-115`):
//! - `95` (valid)
//! - `96` (valid)
//! - `97` (valid)
//! - `98` (valid)
//! - `99`: length 2, prefix "9", "9".repeat(2) == "99". Invalid.
//! - `100` (valid)
//! - `101` (valid)
//! - `102` (valid)
//! - `103` (valid)
//! - `104` (valid)
//! - `105` (valid)
//! - `106` (valid)
//! - `107` (valid)
//! - `108` (valid)
//! - `109` (valid)
//! - `110` (valid)
//! - `111`: length 3, prefix "1", "1".repeat(3) == "111". Invalid.
//! - `112` (valid)
//! - `113` (valid)
//! - `114` (valid)
//! - `115` (valid)
//!
//! ### Complexity Analysis
//! - **Time Complexity:**
//!   - `parse_input`: O(R * L), where R is the number of ranges and L is the average length of a range string.
//!   - `is_invalid_part1`: O(log10(ID)) for string conversion and comparison.
//!   - `is_invalid_part2`: O(log10(ID)^2) in the worst case, due to string conversion and repeated substring comparisons.
//!   - `part_one`/`part_two`: O(SUM(Range_i) * log10(Max_ID)^2), where `SUM(Range_i)` is the total count of numbers to check across all ranges. Given the maximum ID and range sizes, this should be efficient enough.
//! - **Space Complexity:**
//! - O(R) for storing parsed ranges.
//! - O(log10(Max_ID)) for string conversions of IDs.
//!
pub fn part_one(input: &str) -> String {
    let ranges = parse_input(input);
    let mut total_invalid_ids: u64 = 0;

    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_part1(id) {
                total_invalid_ids += id;
            }
        }
    }
    total_invalid_ids.to_string()
}

pub fn part_two(input: &str) -> String {
    let ranges = parse_input(input);
    let mut total_invalid_ids: u64 = 0;

    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_part2(id) {
                total_invalid_ids += id;
            }
        }
    }
    total_invalid_ids.to_string()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter_map(|range_str| {
            let parts: Vec<&str> = range_str.trim().split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<u64>().ok()?;
                let end = parts[1].parse::<u64>().ok()?;
                Some((start, end))
            } else {
                None
            }
        })
        .collect()
}

fn is_invalid_part1(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let (first_half, second_half) = s.split_at(len / 2);
    first_half == second_half
}

fn is_invalid_part2(id: u64) -> bool {
    let s = id.to_string();
    let len_s = s.len();

    // Iterate through all possible substring lengths from 1 up to half the string length
    for sub_len in 1..=(len_s / 2) {
        // Changed this to len_s / 1 as we want to check all possible lengths of sequence
        if len_s.is_multiple_of(sub_len) {
            // If the string length is divisible by sub_len
            let prefix = &s[0..sub_len];
            let repetitions = len_s / sub_len;

            // Check if the string is formed by repeating the prefix
            if s == prefix.repeat(repetitions) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(2);
        assert_eq!(part_one(&input), "1227775554".to_string());
    }

    #[test]
    fn test_part_two() {
        let input = read_example(2);
        assert_eq!(part_two(&input), "4174379265".to_string());
    }

    #[test]
    fn test_is_invalid_part1() {
        assert!(is_invalid_part1(11));
        assert!(is_invalid_part1(6464));
        assert!(is_invalid_part1(123123));
        assert!(!is_invalid_part1(1));
        assert!(!is_invalid_part1(123));
        assert!(!is_invalid_part1(12345));
        assert!(!is_invalid_part1(1234567));
        assert!(!is_invalid_part1(101)); // Not repeated twice
        assert!(!is_invalid_part1(0101)); // Leading zero, not possible with u64
    }

    #[test]
    fn test_is_invalid_part2() {
        assert!(is_invalid_part2(11));
        assert!(is_invalid_part2(22));
        assert!(is_invalid_part2(99));
        assert!(is_invalid_part2(111));
        assert!(is_invalid_part2(999));
        assert!(is_invalid_part2(1010));
        assert!(is_invalid_part2(1188511885));
        assert!(is_invalid_part2(222222));
        assert!(is_invalid_part2(446446));
        assert!(is_invalid_part2(38593859));
        assert!(is_invalid_part2(565656));
        assert!(is_invalid_part2(824824824));
        assert!(is_invalid_part2(2121212121));
        assert!(is_invalid_part2(12341234));
        assert!(is_invalid_part2(123123123));
        assert!(is_invalid_part2(1212121212));
        assert!(is_invalid_part2(1111111));
        assert!(!is_invalid_part2(1));
        assert!(!is_invalid_part2(123));
        assert!(!is_invalid_part2(12345));
        assert!(!is_invalid_part2(1698522));
    }
}
