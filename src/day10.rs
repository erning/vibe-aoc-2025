//! Day 10: Factory
//!
//! Part 1: Toggle lights (XOR in GF(2)). Find min presses using Gaussian elimination.
//! Part 2: Add to counters. Find min presses using linear algebra over integers.

use std::collections::VecDeque;

struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    target_counters: Vec<i64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            // Parse [.##.] pattern
            let bracket_start = line.find('[').unwrap();
            let bracket_end = line.find(']').unwrap();
            let lights_str = &line[bracket_start + 1..bracket_end];
            let target_lights: Vec<bool> =
                lights_str.chars().map(|c| c == '#').collect();

            // Parse (x,y,z) buttons
            let mut buttons: Vec<Vec<usize>> = Vec::new();
            let mut pos = bracket_end + 1;
            while let Some(paren_start) = line[pos..].find('(') {
                let abs_start = pos + paren_start;
                let paren_end =
                    line[abs_start..].find(')').unwrap() + abs_start;
                let button_str = &line[abs_start + 1..paren_end];
                let indices: Vec<usize> = button_str
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                buttons.push(indices);
                pos = paren_end + 1;
            }

            // Parse {x,y,z} counters
            let brace_start = line.find('{').unwrap();
            let brace_end = line.find('}').unwrap();
            let counters_str = &line[brace_start + 1..brace_end];
            let target_counters: Vec<i64> = counters_str
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            Machine {
                target_lights,
                buttons,
                target_counters,
            }
        })
        .collect()
}

fn solve_part1_machine(machine: &Machine) -> u32 {
    let _n_lights = machine.target_lights.len();
    let n_buttons = machine.buttons.len();

    // BFS over button press combinations (each button pressed 0 or 1 times in GF(2))
    // State: bitmask of current light states
    let target: u64 = machine
        .target_lights
        .iter()
        .enumerate()
        .filter(|(_, &on)| on)
        .map(|(i, _)| 1u64 << i)
        .sum();

    let button_masks: Vec<u64> = machine
        .buttons
        .iter()
        .map(|indices| indices.iter().map(|&i| 1u64 << i).sum())
        .collect();

    // BFS: find minimum presses
    let mut visited = std::collections::HashSet::new();
    let mut queue: VecDeque<(u64, u32)> = VecDeque::new();
    queue.push_back((0, 0));
    visited.insert(0u64);

    while let Some((state, presses)) = queue.pop_front() {
        if state == target {
            return presses;
        }

        for &mask in &button_masks {
            let new_state = state ^ mask;
            if visited.insert(new_state) {
                queue.push_back((new_state, presses + 1));
            }
        }
    }

    // If BFS completes without finding target, try brute force over all 2^n combinations
    for combo in 0..(1u64 << n_buttons) {
        let mut state = 0u64;
        let mut presses = 0u32;
        for (i, &mask) in button_masks.iter().enumerate() {
            if combo & (1 << i) != 0 {
                state ^= mask;
                presses += 1;
            }
        }
        if state == target {
            return presses;
        }
    }

    u32::MAX
}

fn solve_part2_machine(machine: &Machine) -> i64 {
    let n_counters = machine.target_counters.len();
    let n_buttons = machine.buttons.len();

    // Create matrix A where A[i][j] = 1 if button j affects counter i
    let mut a: Vec<Vec<i64>> = vec![vec![0; n_buttons]; n_counters];
    for (j, button) in machine.buttons.iter().enumerate() {
        for &idx in button {
            if idx < n_counters {
                a[idx][j] = 1;
            }
        }
    }

    // Use Gaussian elimination over rationals to find the general solution
    // Then search over free variables to minimize sum(x) with x >= 0

    // Augmented matrix [A | b]
    let mut aug: Vec<Vec<(i64, i64)>> = a
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut new_row: Vec<(i64, i64)> =
                row.iter().map(|&x| (x, 1)).collect();
            new_row.push((machine.target_counters[i], 1));
            new_row
        })
        .collect();

    // Gaussian elimination with partial pivoting
    #[allow(clippy::needless_range_loop)]
    let pivot_cols = {
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
                        // aug[r] -= aug[row] * (aug[r][col] / aug[row][col])
                        let (a_num, a_den) = pivot_entry;
                        let (b_num, b_den) = aug[r][col];
                        // factor = b / a = (b_num * a_den) / (b_den * a_num)
                        let factor_num = b_num * a_den;
                        let factor_den = b_den * a_num;

                        for c in 0..=n_buttons {
                            let (row_num, row_den) = aug[row][c];
                            let (r_num, r_den) = aug[r][c];
                            // new = r - row * factor
                            // = r_num/r_den - (row_num/row_den) * (factor_num/factor_den)
                            let new_num =
                                r_num * r_den * row_den * factor_den
                                    - row_num * r_den * factor_num * r_den;
                            let new_den =
                                r_den * r_den * row_den * factor_den;
                            let g = gcd(new_num.abs(), new_den.abs());
                            aug[r][c] = (new_num / g, new_den / g);
                        }
                    }
                }
                row += 1;
            }
        }
        pivot_cols
    };

    // Free variables are columns not in pivot_cols
    let free_cols: Vec<usize> =
        (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    // If no free variables, compute unique solution
    if free_cols.is_empty() {
        let mut solution = vec![0i64; n_buttons];
        for (r, &col) in pivot_cols.iter().enumerate() {
            let (num, den) = aug[r][n_buttons];
            let (pivot_num, pivot_den) = aug[r][col];
            // x[col] = (num/den) / (pivot_num/pivot_den) = num * pivot_den / (den * pivot_num)
            let val_num = num * pivot_den;
            let val_den = den * pivot_num;
            if val_num % val_den != 0 || val_num / val_den < 0 {
                return i64::MAX; // No valid integer solution
            }
            solution[col] = val_num / val_den;
        }
        return solution.iter().sum();
    }

    // Search over free variables
    // For each free variable, try values from 0 to max_target
    let max_val = *machine.target_counters.iter().max().unwrap();

    #[allow(clippy::too_many_arguments)]
    fn search_free(
        idx: usize,
        free_cols: &[usize],
        pivot_cols: &[usize],
        aug: &[Vec<(i64, i64)>],
        n_buttons: usize,
        free_vals: &mut Vec<i64>,
        max_val: i64,
        best: &mut i64,
    ) {
        if idx == free_cols.len() {
            // Compute pivot variable values
            let mut solution = vec![0i64; n_buttons];
            for (i, &fc) in free_cols.iter().enumerate() {
                solution[fc] = free_vals[i];
            }

            for (r, &pc) in pivot_cols.iter().enumerate() {
                // x[pc] = (b - sum of free contributions) / pivot
                let (b_num, b_den) = aug[r][n_buttons];
                let mut rhs_num = b_num;
                let mut rhs_den = b_den;

                for &fc in free_cols {
                    let (coef_num, coef_den) = aug[r][fc];
                    // rhs -= coef * free_val
                    // rhs_num/rhs_den - coef_num/coef_den * free_vals[fc_idx]
                    let fc_idx =
                        free_cols.iter().position(|&x| x == fc).unwrap();
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
                // x[pc] = rhs / pivot = rhs_num * pivot_den / (rhs_den * pivot_num)
                let val_num = rhs_num * pivot_den;
                let val_den = rhs_den * pivot_num;

                if val_den == 0 || val_num % val_den != 0 {
                    return; // Not integer
                }
                let val = val_num / val_den;
                if val < 0 {
                    return; // Negative not allowed
                }
                solution[pc] = val;
            }

            let cost: i64 = solution.iter().sum();
            *best = (*best).min(cost);
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
            );
            free_vals.pop();
        }
    }

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
    );

    best
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part_one(input: &str) -> u32 {
    let machines = parse_input(input);
    machines.iter().map(solve_part1_machine).sum()
}

pub fn part_two(input: &str) -> i64 {
    let machines = parse_input(input);
    machines.iter().map(solve_part2_machine).sum()
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
