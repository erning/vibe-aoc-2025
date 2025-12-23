//! Day 10: Factory
//!
//! Part 1: Find minimum button presses to configure indicator lights (toggle
//! mode).
//! Part 2: Find minimum button presses to configure joltage counters (increment
//! mode).
//!
//! ## Solution Approach
//!
//! **Part 1 Strategy**: BFS for small button counts, greedy fallback.
//! **Part 2 Strategy**: Linear Diophantine optimization. Use constraint
//! propagation with row-by-row elimination for efficiency.

#[derive(Debug, Clone)]
struct Machine {
    target_lights: Vec<bool>,
    target_joltage: Vec<i64>,
    buttons: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let lights_start = line.find('[').unwrap();
            let lights_end = line.find(']').unwrap();
            let lights_str = &line[lights_start + 1..lights_end];

            let target_lights: Vec<bool> =
                lights_str.chars().map(|c| c == '#').collect();

            let mut buttons = Vec::new();
            let mut rest = &line[lights_end + 1..];

            while let Some(paren_start) = rest.find('(') {
                let paren_end = rest.find(')').unwrap();
                let button_str = &rest[paren_start + 1..paren_end];

                let indices: Vec<usize> = button_str
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();

                buttons.push(indices);
                rest = &rest[paren_end + 1..];
            }

            let joltage_start = line.find('{').unwrap();
            let joltage_end = line.find('}').unwrap();
            let joltage_str = &line[joltage_start + 1..joltage_end];

            let target_joltage: Vec<i64> =
                joltage_str.split(',').map(|s| s.trim().parse().unwrap())
                    .collect();

            Machine {
                target_lights,
                target_joltage,
                buttons,
            }
        })
        .collect()
}

// ============== PART 1 ==============

fn solve_lights(machine: &Machine) -> usize {
    let target = &machine.target_lights;

    if target.is_empty() {
        return 0;
    }

    let num_buttons = machine.buttons.len();

    if target.iter().all(|&x| !x) {
        return 0;
    }

    if num_buttons <= 20 {
        bfs_solve_lights(machine)
    } else {
        iterative_solve_lights(machine)
    }
}

fn bfs_solve_lights(machine: &Machine) -> usize {
    use std::collections::VecDeque;

    let start = vec![false; machine.target_lights.len()];
    let target = &machine.target_lights;
    let num_buttons = machine.buttons.len();

    let mut queue: VecDeque<(Vec<bool>, Vec<bool>)> = VecDeque::new();
    let mut visited: std::collections::HashSet<Vec<bool>> =
        std::collections::HashSet::new();

    queue.push_back((vec![false; num_buttons], start.clone()));
    visited.insert(start.clone());

    while let Some((pressed, state)) = queue.pop_front() {
        if state == *target {
            return pressed.iter().filter(|&&x| x).count();
        }

        for i in 0..num_buttons {
            if pressed[i] {
                continue;
            }

            let mut new_state = state.clone();
            for &light_idx in &machine.buttons[i] {
                if light_idx < new_state.len() {
                    new_state[light_idx] = !new_state[light_idx];
                }
            }

            if visited.insert(new_state.clone()) {
                let mut new_pressed = pressed.clone();
                new_pressed[i] = true;
                queue.push_back((new_pressed, new_state));
            }
        }
    }

    iterative_solve_lights(machine)
}

fn iterative_solve_lights(machine: &Machine) -> usize {
    let mut state = vec![false; machine.target_lights.len()];
    let mut presses = 0;
    let target = &machine.target_lights;

    for _ in 0..1000 {
        if state == *target {
            return presses;
        }

        let mut best_button: Option<usize> = None;
        let mut best_score = 0i32;

        for (i, button) in machine.buttons.iter().enumerate() {
            let mut score = 0i32;

            for &light_idx in button {
                if light_idx >= state.len() {
                    continue;
                }

                let current = state[light_idx];
                let goal = target[light_idx];

                if !current && goal {
                    score += 1;
                } else if current && !goal {
                    score -= 1;
                }
            }

            if score > best_score {
                best_score = score;
                best_button = Some(i);
            }
        }

        if best_score <= 0 {
            let mut found = false;

            for (i, button) in machine.buttons.iter().enumerate() {
                let mut affects_wrong = false;

                for &light_idx in button {
                    if light_idx < state.len() && state[light_idx] != target[light_idx]
                    {
                        affects_wrong = true;
                        break;
                    }
                }

                if affects_wrong {
                    best_button = Some(i);
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }
        }

        if let Some(i) = best_button {
            for &light_idx in &machine.buttons[i] {
                if light_idx < state.len() {
                    state[light_idx] = !state[light_idx];
                }
            }
            presses += 1;
        } else {
            break;
        }
    }

    presses
}

// ============== PART 2 ==============

fn solve_joltage(machine: &Machine) -> i64 {
    let m = machine.target_joltage.len();
    let n = machine.buttons.len();

    if m == 0 {
        return 0;
    }

    let a: Vec<Vec<i64>> = (0..m)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if machine.buttons[j].contains(&i) {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();

    let b = &machine.target_joltage;

    ilp_min_sum(&a, b)
}

fn ilp_min_sum(a: &Vec<Vec<i64>>, b: &Vec<i64>) -> i64 {
    let m = b.len();
    let n = a[0].len();

    // Check for simple cases
    if n == 0 {
        return 0;
    }

    // Calculate bounds
    let mut upper_bounds = vec![0i64; n];

    for j in 0..n {
        for i in 0..m {
            if a[i][j] > 0 {
                upper_bounds[j] = upper_bounds[j].max(b[i]);
            }
        }
    }

    // Use row-by-row Gaussian elimination with backtracking
    solve_with_elimination(a, b, &upper_bounds)
}

fn solve_with_elimination(a: &Vec<Vec<i64>>, b: &Vec<i64>, upper_bounds: &Vec<i64>) -> i64 {
    let m = b.len();
    let n = a[0].len();

    // Find row with minimum number of 1s (most constrained)
    let mut min_row = 0;
    let mut min_count = n + 1;

    for i in 0..m {
        let count = (0..n).filter(|&j| a[i][j] > 0).count();

        if count > 0 && count < min_count {
            min_count = count;
            min_row = i;
        }
    }

    if min_count == 1 {
        // Only one button affects this counter
        let j = match (0..n).find(|&col| a[min_row][col] > 0) {
            Some(j) => j,
            None => {
                // Shouldn't happen if min_count == 1, but handle gracefully
                return greedy_simple(a, b, upper_bounds);
            }
        };

        // This button must be pressed exactly b[min_row] times
        let x_j = b[min_row];

        if x_j > upper_bounds[j] {
            // Infeasible
            return i64::MAX;
        }

        // Create reduced problem
        let reduced_b: Vec<i64> = (0..m)
            .map(|i| {
                if i == min_row {
                    0
                } else {
                    b[i] - a[i][j] * x_j
                }
            })
            .collect();

        // Check if any values became negative
        if reduced_b.iter().any(|&v| v < 0) {
            return i64::MAX;
        }

        // Check if reduced problem is satisfied
        let all_zero = reduced_b.iter().all(|&v| v == 0);

        if all_zero {
            return x_j;
        }

        // Recursively solve reduced problem
        let reduced_a: Vec<Vec<i64>> = (0..m)
            .map(|i| {
                if i == min_row {
                    vec![0; n - 1]
                } else {
                    let mut row = Vec::with_capacity(n - 1);

                    for col in 0..n {
                        if col != j {
                            row.push(a[i][col]);
                        }
                    }

                    row
                }
            })
            .collect();

        let reduced_upper: Vec<i64> = (0..n)
            .filter(|&col| col != j)
            .map(|col| upper_bounds[col])
            .collect();

        let reduced_result = solve_with_elimination(&reduced_a, &reduced_b, &reduced_upper);

        if reduced_result == i64::MAX {
            return i64::MAX;
        }

        return x_j + reduced_result;
    }

    // No row with single 1, use branch and bound on most constrained row
    let best_branch = branch_on_constrained(a, b, upper_bounds, min_row);

    best_branch
}

fn branch_on_constrained(
    a: &Vec<Vec<i64>>,
    b: &Vec<i64>,
    upper_bounds: &Vec<i64>,
    row: usize,
) -> i64 {
    let n = a[0].len();

    // Find columns with 1s in this row
    let cols: Vec<usize> = (0..n).filter(|&j| a[row][j] > 0).collect();

    if cols.is_empty() {
        if b[row] == 0 {
            // This row is satisfied, solve remaining
            let reduced_b: Vec<i64> = b.iter().enumerate()
                .filter(|&(i, _)| i != row)
                .map(|(_, &v)| v)
                .collect();

            if reduced_b.is_empty() {
                return 0;
            }

            let reduced_a: Vec<Vec<i64>> = a.iter().enumerate()
                .filter(|&(i, _)| i != row)
                .map(|(_, r)| r.clone())
                .collect();

            return solve_with_elimination(&reduced_a, &reduced_b, upper_bounds);
        } else {
            return i64::MAX; // Infeasible
        }
    }

    let mut best = i64::MAX;

    // Try values for the first column in this row
    let col = cols[0];

    // Calculate max possible value for this column
    let max_val = upper_bounds[col].min(b[row]);

    for v in 0..=max_val {
        // Check if this value is compatible with row constraint
        if a[row][col] * v > b[row] {
            break;
        }

        // Create reduced problem
        let reduced_b: Vec<i64> = (0..b.len())
            .map(|i| b[i] - a[i][col] * v)
            .collect();

        if reduced_b.iter().any(|&val| val < 0) {
            continue;
        }

        let reduced_a: Vec<Vec<i64>> = (0..a.len())
            .map(|i| {
                (0..n).filter(|&j| j != col).map(|j| a[i][j]).collect()
            })
            .collect();

        let reduced_upper: Vec<i64> = (0..n)
            .filter(|&j| j != col)
            .map(|j| upper_bounds[j])
            .collect();

        let result = solve_with_elimination(&reduced_a, &reduced_b, &reduced_upper);

        if result != i64::MAX {
            let total = v + result;

            if total < best {
                best = total;
            }
        }
    }

    best
}

fn greedy_simple(a: &Vec<Vec<i64>>, b: &Vec<i64>, upper_bounds: &Vec<i64>) -> i64 {
    let n = a[0].len();
    let m = b.len();

    let mut x = vec![0i64; n];
    let mut current = vec![0i64; m];

    for _ in 0..100000 {
        if current == *b {
            return x.iter().sum();
        }

        let mut best_j: Option<usize> = None;
        let mut best_score = i64::MIN;

        for j in 0..n {
            if x[j] >= upper_bounds[j] {
                continue;
            }

            let mut would_overshoot = false;

            for i in 0..m {
                if a[i][j] > 0 && current[i] >= b[i] {
                    would_overshoot = true;
                    break;
                }
            }

            if would_overshoot {
                continue;
            }

            let mut score = 0i64;

            for i in 0..m {
                if a[i][j] > 0 {
                    score += b[i] - current[i];
                }
            }

            if score > best_score {
                best_score = score;
                best_j = Some(j);
            }
        }

        if let Some(j) = best_j {
            x[j] += 1;

            for i in 0..m {
                current[i] += a[i][j];
            }
        } else {
            break;
        }
    }

    x.iter().sum()
}

pub fn part_one(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(|m| solve_lights(m)).sum()
}

pub fn part_two(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().map(solve_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 7);
        assert_eq!(part_two(&input), 33);
    }
}
