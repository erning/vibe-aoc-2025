//! Day 10: Factory
//!
//! ## Problem Description
//!
//! Part 1: Configure indicator lights (toggle on/off) with minimum button presses.
//! Each button toggles specific lights. Find minimum presses to match target pattern.
//! This is solving a system of linear equations over GF(2) with minimum Hamming weight.
//!
//! Part 2: Configure joltage counters (increment only) with minimum button presses.
//! Each button increments specific counters by 1. Find minimum presses to reach targets.
//! This is an integer linear programming problem.
//!
//! ## Solution Approach
//!
//! **Part 1**: Enumerate all 2^n button combinations (n is small) to find minimum
//! weight solution over GF(2).
//!
//! **Part 2**: Use Gaussian elimination over rationals to solve the system and
//! find the minimum-sum non-negative integer solution.

use std::collections::HashMap;

/// Represents a machine's configuration
struct Machine {
    lights: Vec<bool>,        // Target indicator light pattern
    buttons: Vec<Vec<usize>>, // Which lights/counters each button affects
    joltages: Vec<u64>,       // Target joltage values
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let bracket_start = line.find('[').unwrap();
            let bracket_end = line.find(']').unwrap();
            let lights: Vec<bool> = line[bracket_start + 1..bracket_end]
                .chars()
                .map(|c| c == '#')
                .collect();

            let mut buttons = Vec::new();
            let mut i = bracket_end + 1;
            let chars: Vec<char> = line.chars().collect();
            while i < chars.len() {
                if chars[i] == '(' {
                    let paren_end = line[i..].find(')').unwrap() + i;
                    let content = &line[i + 1..paren_end];
                    let indices: Vec<usize> = content
                        .split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect();
                    buttons.push(indices);
                    i = paren_end + 1;
                } else if chars[i] == '{' {
                    break;
                } else {
                    i += 1;
                }
            }

            let brace_start = line.find('{').unwrap();
            let brace_end = line.find('}').unwrap();
            let joltages: Vec<u64> = line[brace_start + 1..brace_end]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

/// Solve Part 1 for a single machine using brute force enumeration over GF(2)
fn solve_part1_machine(machine: &Machine) -> u64 {
    let n_lights = machine.lights.len();
    let n_buttons = machine.buttons.len();

    let target: u64 = machine
        .lights
        .iter()
        .enumerate()
        .fold(0u64, |acc, (i, &b)| if b { acc | (1 << i) } else { acc });

    let button_masks: Vec<u64> = machine
        .buttons
        .iter()
        .map(|indices| {
            indices.iter().fold(0u64, |acc, &i| {
                if i < n_lights {
                    acc | (1 << i)
                } else {
                    acc
                }
            })
        })
        .collect();

    let mut min_presses = u64::MAX;

    for mask in 0u64..(1 << n_buttons) {
        let mut state = 0u64;
        let mut presses = 0u64;

        for (i, &button_mask) in button_masks.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                state ^= button_mask;
                presses += 1;
            }
        }

        if state == target && presses < min_presses {
            min_presses = presses;
        }
    }

    if min_presses == u64::MAX {
        0
    } else {
        min_presses
    }
}

/// Rational number for exact arithmetic
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    fn new(num: i64, den: i64) -> Self {
        if den == 0 {
            panic!("Division by zero");
        }
        let g = gcd(num.abs(), den.abs());
        let sign = if den < 0 { -1 } else { 1 };
        Rational {
            num: sign * num / g,
            den: sign * den / g,
        }
    }

    fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }

    fn is_zero(&self) -> bool {
        self.num == 0
    }

    fn is_non_negative_integer(&self) -> bool {
        self.den == 1 && self.num >= 0
    }

    fn as_i64(self) -> Option<i64> {
        if self.den == 1 {
            Some(self.num)
        } else {
            None
        }
    }
}

impl std::ops::Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Rational::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl std::ops::Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.max(1)
    } else {
        gcd(b, a % b)
    }
}

/// Solve Part 2 for a single machine
fn solve_part2_machine(machine: &Machine) -> u64 {
    let n_counters = machine.joltages.len();
    let n_buttons = machine.buttons.len();

    if n_buttons == 0 || machine.joltages.iter().all(|&j| j == 0) {
        return 0;
    }

    // Build matrix A[counter][button] = 1 if button affects counter
    let mut matrix: Vec<Vec<Rational>> =
        vec![vec![Rational::zero(); n_buttons + 1]; n_counters];

    for c in 0..n_counters {
        matrix[c][n_buttons] = Rational::new(machine.joltages[c] as i64, 1);
    }

    for (b, indices) in machine.buttons.iter().enumerate() {
        for &c in indices {
            if c < n_counters {
                matrix[c][b] = Rational::new(1, 1);
            }
        }
    }

    // Gaussian elimination to RREF
    let mut pivot_col = 0;
    let mut pivot_rows = Vec::new();

    for row in 0..n_counters {
        while pivot_col < n_buttons {
            let mut pivot_row = None;
            for r in row..n_counters {
                if !matrix[r][pivot_col].is_zero() {
                    pivot_row = Some(r);
                    break;
                }
            }

            if let Some(pr) = pivot_row {
                matrix.swap(row, pr);
                pivot_rows.push((row, pivot_col));

                let pivot_val = matrix[row][pivot_col];
                for c in 0..=n_buttons {
                    matrix[row][c] = matrix[row][c] / pivot_val;
                }

                for r in 0..n_counters {
                    if r != row && !matrix[r][pivot_col].is_zero() {
                        let factor = matrix[r][pivot_col];
                        for c in 0..=n_buttons {
                            let sub = matrix[row][c] * factor;
                            matrix[r][c] = matrix[r][c] - sub;
                        }
                    }
                }

                pivot_col += 1;
                break;
            } else {
                pivot_col += 1;
            }
        }
    }

    // Find free variables
    let pivot_cols: Vec<usize> = pivot_rows.iter().map(|&(_, c)| c).collect();
    let free_vars: Vec<usize> =
        (0..n_buttons).filter(|c| !pivot_cols.contains(c)).collect();

    let pivot_col_to_row: HashMap<usize, usize> =
        pivot_rows.iter().map(|&(r, c)| (c, r)).collect();

    // Search for minimum sum solution
    find_min_sum_solution(&matrix, &pivot_col_to_row, &free_vars, n_buttons)
}

/// Find the minimum-sum non-negative integer solution
fn find_min_sum_solution(
    matrix: &[Vec<Rational>],
    pivot_col_to_row: &HashMap<usize, usize>,
    free_vars: &[usize],
    n_buttons: usize,
) -> u64 {
    // Estimate max value for free variables based on RHS values
    let max_rhs = matrix
        .iter()
        .map(|row| row.last().unwrap().num.abs())
        .max()
        .unwrap_or(0) as u64;

    // For each free variable, we need to bound its range
    // Basic variables = RHS - sum(coef * free_var)
    // For basic var to be non-negative, we get constraints on free vars

    let mut best_sum = u64::MAX;
    let n_free = free_vars.len();

    if n_free == 0 {
        // No free variables - unique solution
        let mut solution = vec![Rational::zero(); n_buttons];
        for (&col, &row) in pivot_col_to_row {
            solution[col] = matrix[row][n_buttons];
        }
        if solution.iter().all(|r| r.is_non_negative_integer()) {
            return solution
                .iter()
                .filter_map(|r| r.as_i64())
                .map(|v| v as u64)
                .sum();
        }
        return 0;
    }

    // Enumerate free variable assignments with pruning
    let max_search = (max_rhs + 50).min(500); // Reasonable upper bound

    fn search(
        free_vars: &[usize],
        idx: usize,
        assignment: &mut Vec<i64>,
        matrix: &[Vec<Rational>],
        pivot_col_to_row: &HashMap<usize, usize>,
        n_buttons: usize,
        max_val: u64,
        best_sum: &mut u64,
    ) {
        // Early termination if current partial sum is too large
        let partial_sum: i64 = assignment.iter().take(idx).sum();
        if partial_sum < 0 || partial_sum as u64 >= *best_sum {
            return;
        }

        if idx == free_vars.len() {
            // Compute basic variables
            let mut solution = vec![Rational::zero(); n_buttons];
            for (i, &fv) in free_vars.iter().enumerate() {
                solution[fv] = Rational::new(assignment[i], 1);
            }

            for (&col, &row) in pivot_col_to_row {
                let mut val = matrix[row][n_buttons];
                for (i, &fv) in free_vars.iter().enumerate() {
                    let coef = matrix[row][fv];
                    let fv_val = Rational::new(assignment[i], 1);
                    val = val - coef * fv_val;
                }
                solution[col] = val;
            }

            if solution.iter().all(|r| r.is_non_negative_integer()) {
                let sum: u64 = solution
                    .iter()
                    .filter_map(|r| r.as_i64())
                    .map(|v| v as u64)
                    .sum();
                *best_sum = (*best_sum).min(sum);
            }
            return;
        }

        // Try values for current free variable
        for v in 0..=max_val as i64 {
            assignment[idx] = v;
            search(
                free_vars,
                idx + 1,
                assignment,
                matrix,
                pivot_col_to_row,
                n_buttons,
                max_val,
                best_sum,
            );

            // Pruning: if we found a good solution, limit further search
            if *best_sum <= partial_sum as u64 + v as u64 {
                break;
            }
        }
    }

    let mut assignment = vec![0i64; n_free];
    search(
        free_vars,
        0,
        &mut assignment,
        matrix,
        pivot_col_to_row,
        n_buttons,
        max_search,
        &mut best_sum,
    );

    if best_sum == u64::MAX {
        0
    } else {
        best_sum
    }
}

pub fn part_one(input: &str) -> u64 {
    let machines = parse_input(input);
    machines.iter().map(solve_part1_machine).sum()
}

pub fn part_two(input: &str) -> u64 {
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
