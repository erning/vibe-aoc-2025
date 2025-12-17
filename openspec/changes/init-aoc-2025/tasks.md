# Tasks: Initialize Advent of Code 2025

## 1. Update Year Placeholders
- [x] 1.1 Update `Cargo.toml` package name from `aoc-xxxx-in-rust` to `aoc-2025-in-rust`
- [x] 1.2 Update `README.md` title
- [x] 1.3 Update `AGENTS.md` URLs and references
- [x] 1.4 Update `flake.nix` description
- [x] 1.5 Update `openspec/project.md` package name reference

## 2. Update Scripts
- [x] 2.1 Update `scripts/download-input.sh` YEAR variable to 2025
- [x] 2.2 Update `scripts/submit-answer.sh` YEAR variable to 2025

## 3. Rename Day01 to Day00 (Template Reference)
- [x] 3.1 Rename `src/day01.rs` to `src/day00.rs`
- [x] 3.2 Update `src/day00.rs` comments to indicate it's a template/reference
- [x] 3.3 Update `src/lib.rs` module declaration from `day01` to `day00`
- [x] 3.4 Update `src/main.rs` puzzle registration (comment out day00)
- [x] 3.5 Rename `inputs/01-example.txt` to `inputs/00-example.txt`
- [x] 3.6 Rename `inputs/01-input.txt` to `inputs/00-input.txt`

## 4. Update Documentation
- [x] 4.1 Update `AGENTS.md` to reflect 12-day puzzle structure (Day 1-12)
- [x] 4.2 Regenerate `Cargo.lock` with new package name

## 5. Validation
- [x] 5.1 Run `cargo check` to verify compilation
- [x] 5.2 Run `cargo test` to verify tests pass
- [x] 5.3 Run `cargo clippy` to verify no lint warnings