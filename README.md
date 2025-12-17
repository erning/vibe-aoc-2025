# aoc-2025-in-rust

Rust solutions for [Advent of Code 2025](https://adventofcode.com/2025/).

## Solutions

Days 1-12 implemented.

```text
$ cargo run --release --

--- Day 1: Secret Entrance ---
Part One: 1123
Part Two: 6695

--- Day 2: Gift Shop ---
Part One: 19386344315
Part Two: 34421651192

--- Day 3: Lobby ---
Part One: 16927
Part Two: 167384358365132

--- Day 4: Printing Department ---
Part One: 1527
Part Two: 8690

--- Day 5: Cafeteria ---
Part One: 558
Part Two: 344813017450467

--- Day 6: Trash Compactor ---
Part One: 5977759036837
Part Two: 9630000828442

--- Day 7: Laboratories ---
Part One: 1516
Part Two: 1393669447690

--- Day 8: Playground ---
Part One: 62186
Part Two: 8420405530

--- Day 9: Movie Theater ---
Part One: 4729332959
Part Two: 1474477524

--- Day 10: Factory ---
Part One: 524
Part Two: 21696

--- Day 11: Reactor ---
Part One: 607
Part Two: 506264456238938

--- Day 12: Christmas Tree Farm ---
Part One: 599
Part Two: 0
```

## Usage

```bash
# Run all days
cargo run --release --

# Run specific days
cargo run --release -- 1 5 10

# Use example inputs
cargo run --release -- --example

# Run tests
cargo test
```

## Project Structure

```
src/
├── main.rs      # Entry point with puzzle registry
├── lib.rs       # I/O utilities and module exports
└── day{NN}.rs   # Individual day solutions

inputs/
├── {NN}-example.txt   # Example inputs from puzzle descriptions
└── {NN}-input.txt     # Actual puzzle inputs
```
