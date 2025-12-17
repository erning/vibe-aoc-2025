//! # Day 1: Secret Entrance
//!
//! ## Problem Description
//! The Elves have good news and bad news.
//!
//! The good news is that they've discovered [project management](https://en.wikipedia.org/wiki/Project_management)! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.
//!
//! The bad news is that they've realized they have a _different_ emergency: according to their resource planning, none of them have any time left to decorate the North Pole!
//!
//! To save Christmas, the Elves need _you_ to _finish decorating the North Pole by December 12th_.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants _one star_. Good luck!
//!
//! You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the _password_ seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:
//!
//! "Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."
//!
//! The safe has a dial with only an arrow on it; around the dial are the numbers `0` through `99` in order. As you turn the dial, it makes a small _click_ noise as it reaches each number.
//!
//! The attached document (your puzzle input) contains a sequence of _rotations_, one per line, which tell you how to open the safe. A rotation starts with an `L` or `R` which indicates whether the rotation should be to the _left_ (toward lower numbers) or to the _right_ (toward higher numbers). Then, the rotation has a _distance_ value which indicates how many clicks the dial should be rotated in that direction.
//!
//! So, if the dial were pointing at `11`, a rotation of `R8` would cause the dial to point at `19`. After that, a rotation of `L19` would cause it to point at `0`.
//!
//! Because the dial is a circle, turning the dial _left from `0`_ one click makes it point at `99`. Similarly, turning the dial _right from `99`_ one click makes it point at `0`.
//!
//! So, if the dial were pointing at `5`, a rotation of `L10` would cause it to point at `95`. After that, a rotation of `R5` could cause it to point at `0`.
//!
//! The dial starts by pointing at `50`.
//!
//! You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is _the number of times the dial is left pointing at `0` after any rotation in the sequence_.
//!
//! For example, suppose the attached document contained the following rotations:
//!
//! ```
//! L68
//! L30
//! R48
//! L5
//! R60
//! L55
//! L1
//! L99
//! R14
//! L82
//! ```
//!
//! Following these rotations would cause the dial to move as follows:
//!
//! - The dial starts by pointing at `50`.
//! - The dial is rotated `L68` to point at `82`.
//! - The dial is rotated `L30` to point at `52`.
//! - The dial is rotated `R48` to point at `_0_`.
//! - The dial is rotated `L5` to point at `95`.
//! - The dial is rotated `R60` to point at `55`.
//! - The dial is rotated `L55` to point at `_0_`.
//! - The dial is rotated `L1` to point at `99`.
//! - The dial is rotated `L99` to point at `_0_`.
//! - The dial is rotated `R14` to point at `14`.
//! - The dial is rotated `L82` to point at `32`.
//!
//! Because the dial points at `0` a total of three times during this process, the password in this example is `_3_`.
//!
//! Analyze the rotations in your attached document. _What's the actual password to open the door?_
//!
//! ## Part Two
//! You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.
//!
//! As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:
//!
//! "Due to newer security protocols, please use _password method <span title="You should have seen the chaos when the Elves overflowed their 32-bit password method counter.">0x434C49434B</span>_ until further notice."
//!
//! You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times _any click_ causes the dial to point at `0`, regardless of whether it happens during a rotation or at the end of one.
//!
//! Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:
//!
//! - The dial starts by pointing at `50`.
//! - The dial is rotated `L68` to point at `82`; during this rotation, it points at `0` _once_.
//! - The dial is rotated `L30` to point at `52`.
//! - The dial is rotated `R48` to point at `_0_`.
//! - The dial is rotated `L5` to point at `95`.
//! - The dial is rotated `R60` to point at `55`; during this rotation, it points at `0` _once_.
//! - The dial is rotated `L55` to point at `_0_`.
//! - The dial is rotated `L1` to point at `99`.
//! - The dial is rotated `L99` to point at `_0_`.
//! - The dial is rotated `R14` to point at `14`.
//! - The dial is rotated `L82` to point at `32`; during this rotation, it points at `0` _once_.
//!
//! In this example, the dial points at `0` three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be `_6_`.
//!
//! Be careful: if the dial were pointing at `50`, a single rotation like `R1000` would cause the dial to point at `0` ten times before returning back to `50`!
//!
//! Using password method 0x434C49434B, _what is the password to open the door?_
//!
//! ## Solution Approach
//! ### Part One
//! The problem asks us to simulate a dial with numbers `0` to `99`. The dial starts at `50`. We are given a sequence of rotations, each specified by a direction (`L` or `R`) and a distance. We need to count how many times the dial points at `0` _after_ a rotation in the sequence.
//! The dial is circular, so moving left from `0` goes to `99` and right from `99` goes to `0`. This implies modulo arithmetic with `100`.
//!
//! The state of the dial can be represented by a single integer `current_position` from `0` to `99`.
//!
//! For each rotation:
//! 1. Parse the direction and distance.
//! 2. If direction is `L`, subtract distance from `current_position`.
//! 3. If direction is `R`, add distance to `current_position`.
//! 4. Apply modulo `100` to `current_position` to handle the circular nature.
//!    A common way to handle negative results from modulo in Rust (or many languages) is `(current_position % 100 + 100) % 100` for left rotations and `current_position % 100` for right rotations.
//!    Alternatively, for left rotations, if `current_position` becomes negative, add `100` until it's non-negative. For right rotations, if `current_position` exceeds `99`, subtract `100` until it's within `0-99`.
//! 5. Check if `current_position` is `0`. If so, increment a counter.
//!
//! ### Part Two
//! Part Two changes the counting criteria. We now need to count every time the dial clicks past `0` _during_ a rotation, not just at the end.
//! This means for each rotation, we need to consider the path the dial takes.
//!
//! Let `start_position` be the position before the rotation and `end_position` be the position after the rotation (calculated as in Part One).
//! Let `distance` be the absolute value of the rotation amount.
//!
//! If rotating left:
//! The dial moves from `start_position` down to `0`, then `99`, `98`, etc.
//! The number of times it clicks `0` is the number of times `current_position - k * 100` is `0` for `k >= 0`.
//! We need to count `0` if `start_position >= 0` and the rotation passes through `0`.
//! It passes through `0` when `start_position - distance` crosses `0` or `100` boundaries.
//!
//! A simpler approach for counting clicks of `0`:
//! For each rotation, calculate the total change in position.
//! If moving left: `start_position - distance`. The number of times it passes `0` is `floor(start_position / 100)` minus `floor((start_position - distance) / 100)`. If `start_position` itself is `0`, and we move left, it passes `0` again.
//!
//! Let's consider `start_pos` and `end_pos` (modulo 100).
//! For each step (click) of the rotation:
//!   - Decrement/increment current position.
//!   - If `current_position` becomes `0`, increment a counter.
//!     This simulation is too slow for large distances (e.g., `R1000`).
//!
//! A better approach for counting clicks of `0` _during_ a rotation:
//!
//! Let `current_pos` be the dial's position before the rotation.
//! Let `rotation_amount` be the distance value.
//!
//! If rotating right:
//!   The dial moves from `current_pos` to `current_pos + rotation_amount`.
//!   The number of times it crosses `0` (from `99` to `0`) is:
//!   `(current_pos + rotation_amount) / 100` (integer division) - `current_pos / 100`
//!   If `current_pos` is `0` at the start of the rotation, we also count it if the `rotation_amount` is positive. No, the problem says "any click causes the dial to point at 0". So if it starts at 0, that's already pointing at 0. If it starts at 0 and we move R1000, it should count 10 times.
//!   If `current_pos == 0`, and `rotation_amount > 0`, it means it's starting at 0, and immediately moves away, so it doesn't count the initial 0 as "a click causes it to point at 0". It counts when it wraps around.
//!   The number of times `0` is hit when moving right:
//!   `floor((current_pos + rotation_amount - 1) / 100)` - `floor((current_pos - 1) / 100)`
//!   This formula handles the edge cases by shifting the range.
//!
//! If rotating left:
//!   The dial moves from `current_pos` to `current_pos - rotation_amount`.
//!   The number of times it crosses `0` (from `1` to `0`) is:
//!   `floor(current_pos / 100)` - `floor((current_pos - rotation_amount) / 100)`
//!   Again, this assumes integer arithmetic for division.
//!   Number of times `0` is hit when moving left:
//!   `floor(current_pos / 100)` - `floor((current_pos - rotation_amount - 1) / 100)`
//!   The total number of times `0` is hit during the rotation needs to be added to the counter.
//!
//! Example walkthrough for Part 2:
//! Starts at 50.
//! L68: 50 - 68 = -18. ((-18 % 100) + 100) % 100 = 82.
//! Count for L68: from 50 to 82. It passes 0 (from 1 to 0) if it goes `50 -> ... -> 1 -> 0 -> 99 -> ... -> 82`.
//! Number of times it hits 0: `floor(50/100)` - `floor((50 - 68 - 1) / 100)` = `0 - floor(-19/100)` = `0 - (-1)` = `1`. Correct.
//!
//! L30: 82 - 30 = 52. No wrap around. Count = 0.
//!
//! R48: 52 + 48 = 100. (100 % 100) = 0.
//! Count for R48: from 52 to 0. It passes 0 (from 99 to 0) if it goes `52 -> ... -> 99 -> 0`.
//! Number of times it hits 0: `floor((52 + 48 - 1) / 100)` - `floor((52 - 1) / 100)` = `floor(99/100)` - `floor(51/100)` = `0 - 0` = `0`. This is incorrect according to the example which states: "R48 to point at _0_". The example explanation for Part 2 says this one *doesn't* hit 0 during rotation, but ends at 0.
//! The example says "during this rotation, it points at 0 once" for L68, R60, L82. And "R48 to point at _0_". This means the *final position* of 0 is counted for the "end of rotation" part, but not for "during a rotation".
//!
//! The prompt says "count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one."
//! So, if `end_pos == 0`, it counts. If it passed `0` _before_ `end_pos` and `end_pos != 0`, it counts.
//! If it starts at `50` and rotates `R1000`, it should count 10 times.
//!
//! Let's rethink the Part 2 counting:
//!
//! Total clicks of 0 = `current_zero_crossings`
//! `pos` is the current position on the dial.
//! For each `(direction, distance)`:
//!   If `direction == R`:
//!     For `i` from `1` to `distance`:
//!       `pos = (pos + 1) % 100`
//!       If `pos == 0`: `current_zero_crossings += 1`
//!   If `direction == L`:
//!     For `i` from `1` to `distance`:
//!       `pos = (pos - 1 + 100) % 100`
//!       If `pos == 0`: `current_zero_crossings += 1`
//! This simulation is too slow for `R1000`.
//!
//! A mathematical approach for Part 2 counting:
//!
//! Let `start_pos` be the current position.
//! Let `amount` be the rotation distance.
//!
//! If `direction == R`:
//!   `num_zero_crossings_in_rotation = (start_pos + amount) / 100 - start_pos / 100`
//!   If `start_pos == 0`, then it's already at 0. If it moves `R`, it moves away from 0. So it would need to make a full circle to hit 0 again.
//!   The number of times it hits 0 when moving from `start_pos` to `start_pos + amount` is `floor((start_pos + amount - 1) / 100) - floor((start_pos - 1) / 100)`
//!   Let's use `value` to represent the "unwrapped" position. `value_start = start_pos`. `value_end = start_pos + amount` (for R) or `start_pos - amount` (for L).
//!   The number of times `0` is hit *during* the rotation is the number of multiples of 100 between `value_start` and `value_end` (exclusive of `value_start` and inclusive of `value_end`).
//!   If `direction == R`:
//!     `num_zero_crossings = (start_pos + amount - 1) / 100 - (start_pos - 1) / 100`
//!     (This assumes `start_pos >= 0`. If `start_pos=0`, `start_pos-1` becomes -1, `(-1)/100` is 0 in integer division).
//!     To be more robust for `start_pos = 0`: `(start_pos + amount - 1) / 100 - if start_pos == 0 { -1 } else { (start_pos - 1) } / 100`
//!     A simpler way:
//!     Let `total_distance_traveled = amount`.
//!     Let `effective_start_pos = current_pos`.
//!     If `effective_start_pos == 0`, treat it as `100` for counting cycles to avoid double counting the initial `0`. (No, this is wrong, a click causes it to point at 0).
//!
//! Let's reconsider the problem statement: "count the number of times _any click_ causes the dial to point at `0`". This means if `pos == 0` at the start, and it moves, that doesn't count. It only counts if a _click_ results in `pos == 0`.
//!
//! So for `L68` from `50` to `82`:
//!   The value goes from `50` down to `50 - 68 = -18`.
//!   The numbers hit are `49, 48, ..., 1, 0, 99, ..., 82`.
//!   It hits `0` exactly once.
//!   This happens when `start_pos >= 0` and `start_pos - amount < 0`. More precisely, it crosses `0` when `start_pos % 100` is not `0` and `(start_pos - amount) % 100` is `0` or goes past `0`.
//!
//! A solid mathematical way to count zero crossings for a range `[A, B)` (excluding B for right moves, excluding B for left moves) or `(A, B]` (inclusive of B):
//!
//! For R: `current_pos` -> `current_pos + amount`.
//! Number of times `0` is hit when moving from `A` to `B` (inclusive):
//! `(B - 1) / 100 - (A - 1) / 100` (integer division, for `A, B >= 1`)
//! Let `start_unwrapped = current_pos`. `end_unwrapped = current_pos + amount`.
//! If `current_pos` is `0`, then `start_unwrapped` should be `100` for correct cycle counting when moving right.
//!
//! Simpler:
//! Initialize `count_zero_hits = 0`.
//! `current_pos = 50`.
//!
//! For each `(dir, dist)`:
//!   If `dir == 'R'`:
//!     For `_` in `0..dist`:
//!       `current_pos = (current_pos + 1) % 100`
//!       If `current_pos == 0`: `count_zero_hits += 1`
//!   If `dir == 'L'`:
//!     For `_` in `0..dist`:
//!       `current_pos = (current_pos - 1 + 100) % 100`
//!       If `current_pos == 0`: `count_zero_hits += 1`
//!
//! This approach is definitely too slow for R1000.
//!
//! Let's think about `R1000` from `50`.
//! `current_pos` goes `51, ..., 99, 0, 1, ..., 99, 0, ...`
//! Number of times `0` is hit: `1000 / 100 = 10` (if it started at 0 or crosses 0 at the start of a cycle)
//! Or more specifically:
//! `current_pos` (0-99), `dist`.
//!
//! New `current_pos_raw = current_pos + (dist if dir == R else -dist)`.
//! Number of full cycles passed: `abs(current_pos_raw / 100)` ? No.
//!
//! Let's analyze the transitions over `0`.
//!
//! If `dir == 'R'`:
//!   The values pass `0` when the unwrapped position crosses a multiple of `100` from below.
//!   Consider the values `current_pos + 1, current_pos + 2, ..., current_pos + dist`.
//!   How many of these are multiples of `100`?
//!   Let `start_unwrapped = current_pos`. `end_unwrapped = current_pos + dist`.
//!   The multiples of `100` between `start_unwrapped` (exclusive) and `end_unwrapped` (inclusive) are `ceil((start_unwrapped + 1) / 100.0) * 100, ..., floor(end_unwrapped / 100.0) * 100`.
//!   This can be calculated as: `floor(end_unwrapped / 100.0)` - `floor(start_unwrapped / 100.0)` (if start is not multiple of 100)
//!   Or `(end_unwrapped - 1) / 100 - (start_unwrapped - 1) / 100` to make it simpler using integer division properties.
//!
//! Example: `R60` from `55`. `start_unwrapped = 55`, `end_unwrapped = 115`.
//! Number of hits: `(115 - 1) / 100 - (55 - 1) / 100 = 114 / 100 - 54 / 100 = 1 - 0 = 1`. Correct.
//!
//! Example: `R48` from `52`. `start_unwrapped = 52`, `end_unwrapped = 100`.
//! Number of hits: `(100 - 1) / 100 - (52 - 1) / 100 = 99 / 100 - 51 / 100 = 0 - 0 = 0`. Correct, it lands exactly on 0 but doesn't "pass" it during the move (from 99 to 0).
//!
//! If `dir == 'L'`:
//!   The values pass `0` when the unwrapped position crosses a multiple of `100` from above.
//!   Consider the values `current_pos - 1, current_pos - 2, ..., current_pos - dist`.
//!   Let `start_unwrapped = current_pos`. `end_unwrapped = current_pos - dist`.
//!   We are looking for values `..., 101, 100, 99, ..., 1, 0, -1, ...`
//!   The multiples of `100` (representing `0` on the dial) between `end_unwrapped` (inclusive) and `start_unwrapped` (exclusive) are desired.
//!   This means the number of `0`'s hit as it goes from `start_unwrapped` to `end_unwrapped`.
//!   Example: `L68` from `50`. `start_unwrapped = 50`, `end_unwrapped = -18`.
//!   Hits `0` once.
//!   The formula would be `floor(start_unwrapped / 100.0)` - `floor(end_unwrapped / 100.0)`.
//!   `floor(50/100) - floor(-18/100) = 0 - (-1) = 1`. Correct.
//!
//! So for `part_two`:
//! `current_pos = 50`
//! `total_zero_hits = 0`
//! For each `(dir, dist)`:
//!   If `dir == 'R'`:
//!     `total_zero_hits += (current_pos + dist - 1) / 100 - (current_pos - 1) / 100` (using integer division)
//!     `current_pos = (current_pos + dist) % 100`
//!   If `dir == 'L'`:
//!     `total_zero_hits += current_pos / 100 - (current_pos - dist) / 100` (using integer division)
//!     `current_pos = (current_pos - dist).rem_euclid(100)` (to handle negative results correctly for modulo)
//!
//! Let's re-verify the `R48` from `52` case with the formula:
//! `start_pos = 52`, `dist = 48`.
//! `total_zero_hits += (52 + 48 - 1) / 100 - (52 - 1) / 100`
//! `total_zero_hits += 99 / 100 - 51 / 100 = 0 - 0 = 0`. This means it does not hit 0 *during* the rotation. This seems correct.
//!
//! So the plan is:
//!
//! **Data Structure for parsed input:**
//! A vector of tuples `(char, u32)` representing `(direction, distance)`.
//!
//! **`parse_input` function:**
//! Iterates through lines, parses each `L/R` and the integer distance.
//!
//! **`part_one` function:**
//! Initializes `current_pos = 50`, `zero_count = 0`.
//! For each `(dir, dist)` in parsed input:
//!   Update `current_pos` using modulo `100`.
//!   If `current_pos == 0`, increment `zero_count`.
//! Return `zero_count`.
//!
//! **`part_two` function:**
//! Initializes `current_pos = 50`, `total_zero_hits = 0`.
//! For each `(dir, dist)` in parsed input:
//!   If `dir == 'R'`:
//!     `total_zero_hits += (current_pos + dist - 1) / 100 - (current_pos - 1) / 100`
//!     `current_pos = (current_pos + dist) % 100`
//!   If `dir == 'L'`:
//!     `total_zero_hits += current_pos / 100 - (current_pos - dist) / 100`
//!     `current_pos = (current_pos - dist).rem_euclid(100)`
//! Return `total_zero_hits`.

//! ## Complexity Analysis
//! ### Part One
//! - **Time Complexity**: O(N), where N is the number of rotations. Each rotation involves constant time parsing and arithmetic operations.
//! - **Space Complexity**: O(N) to store the parsed rotations.
//!
//! ### Part Two
//! - **Time Complexity**: O(N), where N is the number of rotations. Each rotation involves constant time arithmetic operations (no simulation of individual clicks).
//! - **Space Complexity**: O(N) to store the parsed rotations.
//!
pub fn part_one(input: &str) -> String {
    let rotations = parse_input(input);
    let mut current_pos = 50;
    let mut zero_count = 0;

    for (dir, dist) in rotations {
        match dir {
            'R' => current_pos = (current_pos + dist) % 100,
            'L' => {
                current_pos =
                    (current_pos as i32 - dist as i32).rem_euclid(100) as u32
            }
            _ => unreachable!(),
        }
        if current_pos == 0 {
            zero_count += 1;
        }
    }
    zero_count.to_string()
}

pub fn part_two(input: &str) -> String {
    let rotations = parse_input(input);
    let mut current_pos = 50;
    let mut total_zero_hits: i32 = 0;

    for (dir, dist) in rotations {
        match dir {
            'R' => {
                let start_i32 = current_pos as i32;
                let dist_i32 = dist as i32;
                let end_i32 = start_i32 + dist_i32;

                // Count how many times it passed 0 (from 99 to 0) during this rotation
                total_zero_hits += (end_i32 - 1).div_euclid(100)
                    - (start_i32 - 1).div_euclid(100);
                current_pos = end_i32.rem_euclid(100) as u32;
            }
            'L' => {
                let start_i32 = current_pos as i32;
                let dist_i32 = dist as i32;
                let end_i32 = start_i32 - dist_i32;

                // Count how many times it passed 0 (from 1 to 0) during this rotation
                total_zero_hits +=
                    start_i32.div_euclid(100) - end_i32.div_euclid(100);
                current_pos = end_i32.rem_euclid(100) as u32;
            }
            _ => unreachable!(),
        }
    }
    total_zero_hits.to_string()
}

fn parse_input(input: &str) -> Vec<(char, u32)> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let dir = line.chars().next()?;
            let dist = line[1..].parse::<u32>().ok()?;
            Some((dir, dist))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example(1);
        assert_eq!(part_one(&input), "3".to_string());
    }

    #[test]
    fn test_part_two() {
        let input = read_example(1);
        assert_eq!(part_two(&input), "6".to_string());
    }
}
