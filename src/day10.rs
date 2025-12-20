//! Day 10: Factory
//!
//! ## Problem Description
//!
//! Minimize button presses to configure indicator lights and
//! joltage levels.
//! - Part 1: Light toggle puzzle (GF(2) linear equations)
//! - Part 2: Joltage counter puzzle (integer linear equations)

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
    // Greedy approach: solve left-to-right using constraint propagation
    let n_buttons = buttons.len();
    let n_counters = target.len();

    let mut presses = vec![0i64; n_buttons];
    let mut current = vec![0i64; n_counters];

    // For each counter, if there's only one button affecting it,
    // we can directly determine that button's count
    // Otherwise, use greedy selection of best button

    // Repeat until converged
    for _ in 0..100 {
        let mut changed = false;

        // For each counter that isn't satisfied
        for c in 0..n_counters {
            if current[c] < target[c] {
                let deficit = target[c] - current[c];

                // Find best button to press (covers most unsatisfied counters)
                let mut best_button = 0;
                let mut best_coverage = 0;

                for b in 0..n_buttons {
                    let mut coverage = 0;
                    for &counter_idx in &buttons[b] {
                        if counter_idx < n_counters
                            && current[counter_idx] < target[counter_idx]
                        {
                            coverage += 1;
                        }
                    }
                    if coverage > best_coverage {
                        best_coverage = coverage;
                        best_button = b;
                    }
                }

                if best_coverage > 0 {
                    presses[best_button] += deficit;
                    for &counter_idx in &buttons[best_button] {
                        if counter_idx < n_counters {
                            current[counter_idx] += deficit;
                        }
                    }
                    changed = true;
                    break;
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
