//! Day 10: Factory
//!
//! ## Problem Description
//!
//! Minimize button presses to configure indicator lights and joltage levels.
//! - Part 1: Achieve target light pattern (toggle parity)
//! - Part 2: Achieve target joltage levels (integer equations)

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_line(line: &str) -> Option<Machine> {
    // Parse format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    let mut parts = line.split_whitespace();

    // Parse lights
    let lights_str = parts.next()?;
    let lights_str = &lights_str[1..lights_str.len() - 1]; // Remove [ ]
    let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

    // Parse buttons
    let mut buttons = Vec::new();
    loop {
        let button_str = parts.next()?;
        if button_str.starts_with('{') {
            // Move to parse joltage
            let joltage_str =
                if button_str.starts_with('{') && button_str.ends_with('}') {
                    button_str
                } else {
                    parts.next()?
                };
            let joltage_str = &joltage_str[1..joltage_str.len() - 1]; // Remove { }
            let joltage: Vec<i64> = joltage_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            return Some(Machine {
                lights,
                buttons,
                joltage,
            });
        }

        let button_str = &button_str[1..button_str.len() - 1]; // Remove ( )
        let button: Vec<usize> = button_str
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        buttons.push(button);
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().filter_map(parse_line).collect()
}

pub fn part_one(input: &str) -> u64 {
    let machines = parse_input(input);
    let mut total = 0u64;

    for machine in machines {
        let presses = solve_lights(&machine.lights, &machine.buttons);
        total += presses;
    }

    total
}

pub fn part_two(input: &str) -> u64 {
    let machines = parse_input(input);
    let mut total = 0u64;

    for machine in machines {
        let presses = solve_joltage(&machine.joltage, &machine.buttons);
        total += presses;
    }

    total
}

fn solve_lights(target: &[bool], buttons: &[Vec<usize>]) -> u64 {
    // Use Gaussian elimination modulo 2 to find minimum button presses
    // This is a system of linear equations over GF(2)

    let n_lights = target.len();
    let n_buttons = buttons.len();

    // Build the system: button_effects[i][j] = 1 if button i affects light j
    let mut effects = vec![vec![0u8; n_lights]; n_buttons];
    for (b_idx, button) in buttons.iter().enumerate() {
        for &light_idx in button {
            if light_idx < n_lights {
                effects[b_idx][light_idx] = 1;
            }
        }
    }

    // Target state (mod 2)
    let target_mod2: Vec<u8> =
        target.iter().map(|&b| if b { 1 } else { 0 }).collect();

    // Use greedy approach: try all combinations up to a reasonable limit
    // For small n_buttons, brute force is viable
    if n_buttons <= 20 {
        let mut min_presses = u64::MAX;

        for mask in 0..(1u64 << n_buttons) {
            let mut state = vec![0u8; n_lights];

            for (b_idx, button) in buttons.iter().enumerate() {
                let count = ((mask >> b_idx) & 1) as usize;
                for &light_idx in button {
                    if light_idx < n_lights {
                        state[light_idx] ^= count as u8;
                    }
                }
            }

            if state == target_mod2 {
                let presses = mask.count_ones() as u64;
                min_presses = min_presses.min(presses);
            }
        }

        return if min_presses == u64::MAX {
            0
        } else {
            min_presses
        };
    }

    0 // Fallback
}

fn solve_joltage(target: &[i64], buttons: &[Vec<usize>]) -> u64 {
    // Solve the system of linear equations to find minimum total presses
    // target[i] = sum(presses[j] for j where button j affects counter i)

    let n_counters = target.len();
    let n_buttons = buttons.len();

    // Build the system: effects[i][j] = 1 if button j affects counter i
    let mut effects = vec![vec![0i64; n_buttons]; n_counters];
    for (b_idx, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                effects[counter_idx][b_idx] = 1;
            }
        }
    }

    // Use a greedy/iterative approach: for each counter, determine needed presses
    // This works if buttons form a triangular or solvable system
    let mut presses = vec![0i64; n_buttons];

    // Try to solve greedily
    for _ in 0..10 {
        let mut changed = false;

        for counter in 0..n_counters {
            let mut current = 0i64;
            for (b_idx, &count) in presses.iter().enumerate() {
                current += effects[counter][b_idx] * count;
            }

            let deficit = target[counter] - current;
            if deficit != 0 {
                // Find a button that affects this counter
                for (b_idx, &effect) in effects[counter].iter().enumerate() {
                    if effect != 0 {
                        let add = deficit / effect;
                        if add > 0 {
                            presses[b_idx] += add;
                            changed = true;
                            break;
                        }
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    presses.iter().sum::<i64>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example_part_one() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 7);
    }

    #[test]
    fn example_part_two() {
        let input = read_example(10);
        assert_eq!(part_two(&input), 33);
    }
}
