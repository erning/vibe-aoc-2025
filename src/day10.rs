//! Day 10: Factory
//!
//! Part 1: Find minimum button presses to configure indicator lights (toggle
//! mode).
//! Part 2: Find minimum button presses to configure joltage counters (increment
//! mode).
//!
//! ## Solution Approach
//!
//! **Part 1 Strategy**: BFS for XOR operations in GF(2).
//! **Part 2 Strategy**: Gaussian elimination over rationals to reduce system,
//! then search over free variables for optimal integer solution.

use std::collections::VecDeque;

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

    // Build augmented matrix [A|b] as rational numbers (numerator, denominator)
    let mut aug: Vec<Vec<(i64, i64)>> = vec![vec![(0, 1); n + 1]; m];

    for i in 0..m {
        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                aug[i][j] = (1, 1);
            }
        }
        aug[i][n] = (machine.target_joltage[i], 1);
    }

    // Gaussian elimination to find pivot columns
    let n_counters = m;
    let n_buttons = n;
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..n_buttons {
        // Find pivot
        let mut pivot_row = None;

        for r in row..n_counters {
            if aug[r][col].0 != 0 {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            aug.swap(row, pr);
            pivot_cols.push(col);

            // Eliminate column
            let pivot_entry = aug[row][col];

            for r in 0..n_counters {
                if r != row && aug[r][col].0 != 0 {
                    let (a_num, a_den) = pivot_entry;
                    let (b_num, b_den) = aug[r][col];
                    let factor_num = b_num * a_den;
                    let factor_den = b_den * a_num;

                    for c in 0..=n_buttons {
                        let (row_num, row_den) = aug[row][c];
                        let (r_num, r_den) = aug[r][c];
                        let new_num =
                            r_num * r_den * row_den * factor_den
                                - row_num * r_den * factor_num * r_den;
                        let new_den = r_den * r_den * row_den * factor_den;
                        let g = gcd(new_num.abs(), new_den.abs());
                        if g > 0 {
                            aug[r][c] = (new_num / g, new_den / g);
                        }
                    }
                }
            }

            row += 1;
        }
    }

    // Free variables
    let free_cols: Vec<usize> =
        (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    let max_val = *machine.target_joltage.iter().max().unwrap_or(&0);

    // If no free variables, unique solution
    if free_cols.is_empty() {
        let mut solution = vec![0i64; n_buttons];

        for (r, &col) in pivot_cols.iter().enumerate() {
            let (num, den) = aug[r][n_buttons];
            let (pivot_num, pivot_den) = aug[r][col];
            let val_num = num * pivot_den;
            let val_den = den * pivot_num;

            if val_den == 0 || val_num % val_den != 0 || val_num / val_den < 0 {
                return greedy_solve(machine);
            }

            solution[col] = val_num / val_den;
        }

        return solution.iter().sum();
    }

    // Search over free variables
    let mut best = i64::MAX;
    let mut free_vals = Vec::new();

    search_free(
        0,
        &free_cols,
        &pivot_cols,
        &aug,
        n_buttons,
        &mut free_vals,
        max_val,
        &mut best,
        machine,
    );

    if best == i64::MAX {
        greedy_solve(machine)
    } else {
        best
    }
}

fn search_free(
    idx: usize,
    free_cols: &[usize],
    pivot_cols: &[usize],
    aug: &[Vec<(i64, i64)>],
    n_buttons: usize,
    free_vals: &mut Vec<i64>,
    max_val: i64,
    best: &mut i64,
    machine: &Machine,
) {
    if idx == free_cols.len() {
        // Compute pivot variable values
        let mut solution = vec![0i64; n_buttons];

        for (i, &fc) in free_cols.iter().enumerate() {
            solution[fc] = free_vals[i];
        }

        for (r, &pc) in pivot_cols.iter().enumerate() {
            let (b_num, b_den) = aug[r][n_buttons];
            let mut rhs_num = b_num;
            let mut rhs_den = b_den;

            for &fc in free_cols {
                let (coef_num, coef_den) = aug[r][fc];
                let fc_idx = free_cols.iter().position(|&x| x == fc).unwrap();
                let sub_num = coef_num * free_vals[fc_idx];
                let sub_den = coef_den;
                rhs_num = rhs_num * sub_den - sub_num * rhs_den;
                rhs_den *= sub_den;
                let g = gcd(rhs_num.abs(), rhs_den.abs());
                if g > 0 {
                    rhs_num /= g;
                    rhs_den /= g;
                }
            }

            let (pivot_num, pivot_den) = aug[r][pc];
            let val_num = rhs_num * pivot_den;
            let val_den = rhs_den * pivot_num;

            if val_den == 0 || val_num % val_den != 0 || val_num / val_den < 0 {
                return;
            }

            solution[pc] = val_num / val_den;
        }

        let cost: i64 = solution.iter().sum();

        if cost < *best {
            *best = cost;
        }

        return;
    }

    let current_cost: i64 = free_vals.iter().sum();

    if current_cost >= *best {
        return;
    }

    for v in 0..=max_val {
        free_vals.push(v);

        search_free(
            idx + 1,
            free_cols,
            pivot_cols,
            aug,
            n_buttons,
            free_vals,
            max_val,
            best,
            machine,
        );

        free_vals.pop();
    }
}

fn greedy_solve(machine: &Machine) -> i64 {
    let m = machine.target_joltage.len();
    let n = machine.buttons.len();

    let mut x = vec![0i64; n];
    let mut current = vec![0i64; m];

    let mut upper_bounds = vec![0i64; n];

    for j in 0..n {
        for i in 0..m {
            if machine.buttons[j].contains(&i) {
                upper_bounds[j] = upper_bounds[j].max(machine.target_joltage[i]);
            }
        }
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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
