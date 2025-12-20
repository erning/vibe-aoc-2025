//! Day 1: Secret Entrance
//!
//! ## Problem Description
//!
//! Part 1: Simulate a dial safe with rotations (L/R with distance).
//! Count how many times the dial points at 0 after each rotation.
//! The dial has positions 0-99 and wraps around.
//!
//! Part 2: Count how many times the dial points at 0 during all
//! rotations, including intermediate positions (every click).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse each line as a direction (L/R) and a
//! distance number.
//!
//! **Part 1 Strategy**: Track the current position starting at 50.
//! For each rotation, apply the move with modulo 100 arithmetic,
//! and count when the result is 0.
//!
//! **Part 2 Strategy**: For each rotation, count how many times
//! position 0 is crossed during the rotation. This includes the
//! final position if it's 0.
//!
//! **Complexity**: O(n) for part 1 where n is the number of
//! rotations. O(n*d) for part 2 where d is the average distance.

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    dir: Direction,
    dist: i32,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|line| {
            let dir = if line.starts_with('L') {
                Direction::Left
            } else {
                Direction::Right
            };
            let dist = line[1..].parse().unwrap();
            Rotation { dir, dist }
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut pos = 50;
    let mut count = 0;

    for rotation in rotations {
        match rotation.dir {
            Direction::Left => {
                pos = (pos - rotation.dist).rem_euclid(100);
            }
            Direction::Right => {
                pos = (pos + rotation.dist).rem_euclid(100);
            }
        }
        if pos == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_two(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut pos: i32 = 50;
    let mut count = 0;

    for rotation in rotations {
        let dist = rotation.dist;

        match rotation.dir {
            Direction::Left => {
                for _ in 0..dist {
                    pos = (pos - 1).rem_euclid(100);
                    if pos == 0 {
                        count += 1;
                    }
                }
            }
            Direction::Right => {
                for _ in 0..dist {
                    pos = (pos + 1).rem_euclid(100);
                    if pos == 0 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 6);
    }
}
