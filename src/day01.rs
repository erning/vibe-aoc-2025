//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! The Elves need help opening a secret entrance. A safe has a dial with numbers
//! 0-99 in a circle. Rotations are specified as L (left/toward lower numbers) or
//! R (right/toward higher numbers) followed by a distance.
//!
//! **Part 1**: The password is the number of times the dial points at 0 AFTER
//! any rotation ends.
//!
//! **Part 2**: Using "password method 0x434C49434B", count ALL times the dial
//! points at 0, including during rotations (not just at the end).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a rotation direction (L/R) and distance.
//!
//! **Part 1 Strategy**: Simulate the dial starting at 50. For each rotation,
//! update the position modulo 100. Count how many times the position is 0 after
//! each rotation completes.
//! **Complexity**: O(n) where n is the number of rotations.
//!
//! **Part 2 Strategy**: Same as Part 1, but additionally count how many times
//! the dial passes through 0 during each rotation. For each click during the
//! rotation, check if it lands on 0.
//! **Complexity**: O(n * d) where n is rotations and d is total distance, which
//! could be slow for large distances. Instead, we can calculate directly:
//! - For a rotation of distance D, the dial passes through 0 exactly D/100 times
//!   (floor division), plus possibly one more time depending on the starting
//!   position and direction.

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let distance = chars.as_str().parse().unwrap();
            (direction, distance)
        })
        .collect()
}

/// Simulate dial rotations and count how many times it lands on 0 after each rotation.
pub fn part_one(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position = 50;
    let mut count = 0;

    for (direction, distance) in rotations {
        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance).rem_euclid(100),
            _ => position,
        };
        if position == 0 {
            count += 1;
        }
    }

    count
}

/// Simulate dial rotations and count ALL times it points at 0, including during rotations.
pub fn part_two(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut position: i32 = 50;
    let mut count = 0;

    for (direction, distance) in rotations {
        let distance = distance.abs();
        match direction {
            'L' => {
                // Moving left (decreasing)
                // Positions visited: P-1, P-2, ..., P-D (mod 100)
                for _ in 1..distance {
                    position = (position - 1).rem_euclid(100);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            'R' => {
                // Moving right (increasing)
                // Positions visited: P+1, P+2, ..., P+D (mod 100)
                for _ in 1..=distance {
                    position = (position + 1).rem_euclid(100);
                    if position == 0 {
                        count += 1;
                    }
                }
            }
            _ => {}
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 3);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(1);
        assert_eq!(part_two(&input), 6);
    }
}
