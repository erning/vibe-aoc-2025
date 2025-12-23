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
//! **Part 2 Strategy**: Linear Diophantine optimization - minimize sum(x) subject
//! to Ax = b, x ≥ 0. Using DFS with aggressive pruning for small problems,
//! constraint-based greedy for larger ones.

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

    // For small problems, use exact DFS
    if n <= 12 && b.iter().sum::<i64>() <= 200 {
        return exact_dfs(&a, b);
    }

    // For larger problems, use greedy solution
    let mut upper_bounds = vec![0i64; n];

    for j in 0..n {
        for i in 0..m {
            if a[i][j] > 0 {
                upper_bounds[j] = upper_bounds[j].max(b[i]);
            }
        }
    }

    greedy_init(&a, b, &upper_bounds).iter().sum()
}

fn exact_dfs(a: &Vec<Vec<i64>>, b: &Vec<i64>) -> i64 {
    let n = a[0].len();

    let mut upper_bounds = vec![0i64; n];

    for j in 0..n {
        for i in 0..b.len() {
            if a[i][j] > 0 {
                upper_bounds[j] = upper_bounds[j].max(b[i]);
            }
        }
    }

    let mut best = i64::MAX;
    let mut current = vec![0i64; n];
    let mut current_vals = vec![0i64; b.len()];

    dfs_search(a, b, &upper_bounds, 0, 0, &mut best, &mut current, &mut current_vals);

    best
}

fn dfs_search(
    a: &Vec<Vec<i64>>,
    b: &Vec<i64>,
    upper_bounds: &Vec<i64>,
    idx: usize,
    current_sum: i64,
    best: &mut i64,
    current: &mut Vec<i64>,
    current_vals: &mut Vec<i64>,
) {
    let n = a[0].len();
    let m = b.len();

    if current_sum >= *best {
        return;
    }

    for i in 0..m {
        if current_vals[i] > b[i] {
            return;
        }

        let max_remaining: i64 = (idx..n).map(|j| a[i][j] * upper_bounds[j]).sum();

        if current_vals[i] + max_remaining < b[i] {
            return;
        }
    }

    if idx == n {
        for i in 0..m {
            if current_vals[i] != b[i] {
                return;
            }
        }

        *best = current_sum;
        return;
    }

    let mut upper = upper_bounds[idx];

    for i in 0..m {
        if a[i][idx] == 0 {
            continue;
        }

        let remaining = b[i] - current_vals[i];

        if remaining >= 0 && a[i][idx] > 0 {
            upper = upper.min(remaining / a[i][idx]);
        }
    }

    for v in 0..=upper {
        for i in 0..m {
            current_vals[i] += a[i][idx] * v;
        }
        current[idx] = v;

        dfs_search(
            a,
            b,
            upper_bounds,
            idx + 1,
            current_sum + v,
            best,
            current,
            current_vals,
        );

        for i in 0..m {
            current_vals[i] -= a[i][idx] * v;
        }
        current[idx] = 0;
    }
}

fn constraint_solve(a: &Vec<Vec<i64>>, b: &Vec<i64>) -> i64 {
    let n = a[0].len();
    let m = b.len();

    let mut upper_bounds = vec![0i64; n];

    for j in 0..n {
        for i in 0..m {
            if a[i][j] > 0 {
                upper_bounds[j] = upper_bounds[j].max(b[i]);
            }
        }
    }

    // Initialize with greedy
    let mut x = greedy_init(a, b, &upper_bounds);

    // Local search: try to improve by adjusting variables
    let mut improved = true;
    let mut iterations = 0;

    while improved && iterations < 1000 {
        iterations += 1;
        improved = false;

        for j in 0..n {
            // Try decreasing x[j] and compensating with others
            if x[j] == 0 {
                continue;
            }

            let old_val = x[j];
            x[j] = 0;

            // Recompute current state
            let mut current = vec![0i64; m];

            for k in 0..n {
                for i in 0..m {
                    current[i] += a[i][k] * x[k];
                }
            }

            // Try to fix using other variables
            let mut fixed = false;

            for k in 0..n {
                if k == j {
                    continue;
                }

                while x[k] < upper_bounds[k] {
                    // Check if pressing k helps
                    let mut would_help = true;

                    for i in 0..m {
                        if a[i][k] > 0 && current[i] >= b[i] {
                            would_help = false;
                            break;
                        }
                    }

                    if !would_help {
                        break;
                    }

                    // Press k
                    for i in 0..m {
                        current[i] += a[i][k];
                    }
                    x[k] += 1;

                    // Check if solved
                    if current == *b {
                        fixed = true;
                        break;
                    }
                }

                if fixed {
                    break;
                }
            }

            if fixed && x.iter().sum::<i64>() < old_val {
                improved = true;
            } else if !fixed {
                // Revert
                x[j] = old_val;
            }
        }
    }

    x.iter().sum()
}

fn greedy_init(a: &Vec<Vec<i64>>, b: &Vec<i64>, upper_bounds: &Vec<i64>) -> Vec<i64> {
    let n = a[0].len();
    let m = b.len();

    let mut x = vec![0i64; n];
    let mut current = vec![0i64; m];

    for _ in 0..100000 {
        if current == *b {
            return x;
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

    x
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
