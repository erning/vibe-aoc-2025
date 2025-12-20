//! Day 10: Factory
//!
//! ## Problem Description
//!
//! Minimize button presses to configure indicator lights and
//! joltage levels.
//! - Part 1: Light toggle puzzle (GF(2) linear equations)
//! - Part 2: Joltage counter puzzle (integer linear equations)

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse_line(line: &str) -> Option<Machine> {
    let mut parts = line.split_whitespace();

    let lights_str = parts.next()?;
    let lights_str = &lights_str[1..lights_str.len() - 1];
    let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

    let mut buttons = Vec::new();
    loop {
        let button_str = parts.next()?;
        if button_str.starts_with('{') {
            let joltage_str = if button_str.ends_with('}') {
                button_str.to_string()
            } else {
                format!("{} {}", button_str, parts.next().unwrap_or(""))
            };
            let joltage_str = &joltage_str[1..joltage_str.len() - 1];
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

        let button_str = &button_str[1..button_str.len() - 1];
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
    let n_lights = target.len();
    let n_buttons = buttons.len();

    let mut min_presses = u64::MAX;

    for mask in 0..(1u64 << n_buttons) {
        let mut state = vec![false; n_lights];

        for (b_idx, button) in buttons.iter().enumerate() {
            if ((mask >> b_idx) & 1) != 0 {
                for &light_idx in button {
                    if light_idx < n_lights {
                        state[light_idx] = !state[light_idx];
                    }
                }
            }
        }

        if state == target {
            let presses = mask.count_ones() as u64;
            min_presses = min_presses.min(presses);
        }
    }

    if min_presses == u64::MAX {
        0
    } else {
        min_presses
    }
}

fn solve_joltage(target: &[i64], buttons: &[Vec<usize>]) -> u64 {
    // Dijkstra's algorithm with Pareto optimization
    // State: (current_presses, counter_values)
    let n_counters = target.len();

    let initial = vec![0i64; n_counters];

    if initial == target {
        return 0;
    }

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0u64), initial.clone()));

    let mut visited = HashMap::new();
    visited.insert(initial, 0u64);

    while let Some((Reverse(presses), state)) = heap.pop() {
        if state == target {
            return presses;
        }

        if let Some(&prev_presses) = visited.get(&state) {
            if prev_presses < presses {
                continue;
            }
        }

        // Try each button
        for button in buttons {
            let mut next_state = state.clone();

            for &c_idx in button {
                if c_idx < n_counters {
                    next_state[c_idx] += 1;
                }
            }

            // Prune aggressively: skip if we're way over target
            let mut overshooting = false;
            for i in 0..n_counters {
                if next_state[i] > target[i] * 2 {
                    overshooting = true;
                    break;
                }
            }

            if overshooting {
                continue;
            }

            let next_presses = presses + 1;

            let should_visit = visited
                .get(&next_state)
                .map_or(true, |&prev| next_presses < prev);

            if should_visit {
                visited.insert(next_state.clone(), next_presses);
                heap.push((Reverse(next_presses), next_state));
            }
        }
    }

    0
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
