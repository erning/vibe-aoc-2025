//! Day 1: Secret Entrance
//!
//! A safe dial puzzle where we rotate a dial left/right and count positions at 0.
//!
//! Part 1: Count times dial ends at 0 after a rotation.
//! Part 2: Count all times dial passes through or lands on 0 during rotations.

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let dist: i32 = line[1..].parse().unwrap();
            (dir, dist)
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let rotations = parse_input(input);
    let mut pos: i32 = 50;
    let mut count = 0;

    for (dir, dist) in rotations {
        match dir {
            'L' => pos = (pos - dist).rem_euclid(100),
            'R' => pos = (pos + dist).rem_euclid(100),
            _ => unreachable!(),
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

    for (dir, dist) in rotations {
        let (new_pos, zeros) = match dir {
            'L' => {
                let new_pos = (pos - dist).rem_euclid(100);
                // Count how many times we click to 0 going left (toward lower numbers)
                // Going left from pos by dist clicks
                // If pos > 0, we hit 0 after exactly pos clicks
                // If pos == 0, first click goes to 99, so we need 100 clicks to return to 0
                let zeros = if pos == 0 {
                    // From 0, going left: 0->99->98->...->0 takes 100 clicks
                    dist / 100
                } else if dist >= pos {
                    // We reach 0 after pos clicks, then every 100 more clicks
                    1 + (dist - pos) / 100
                } else {
                    0
                };
                (new_pos, zeros)
            }
            'R' => {
                let new_pos = (pos + dist).rem_euclid(100);
                // Count how many times we click to 0 going right (toward higher numbers)
                // If pos > 0, we hit 0 after (100 - pos) clicks
                // If pos == 0, first click goes to 1, so we need 100 clicks to return to 0
                let zeros = if pos == 0 {
                    // From 0, going right: 0->1->2->...->0 takes 100 clicks
                    dist / 100
                } else {
                    let clicks_to_zero = 100 - pos;
                    if dist >= clicks_to_zero {
                        1 + (dist - clicks_to_zero) / 100
                    } else {
                        0
                    }
                };
                (new_pos, zeros)
            }
            _ => unreachable!(),
        };
        count += zeros;
        pos = new_pos;
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
