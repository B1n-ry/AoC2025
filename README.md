# AoC2025

Advent of Code 2025 solutions in Rust.

## Project Structure

```
AoC2025/
├── src/
│   ├── main.rs      # Entry point with day dispatcher
│   ├── utils.rs     # Helper functions for reading input files
│   ├── day01.rs     # Day 1 solution
│   ├── day02.rs     # Day 2 solution
│   ├── ...
│   └── day12.rs     # Day 12 solution
├── inputs/
│   ├── day01.txt    # Input for Day 1
│   ├── day02.txt    # Input for Day 2
│   ├── ...
│   └── day12.txt    # Input for Day 12
├── Cargo.toml
└── README.md
```

## Usage

### Building the Project

```bash
cargo build
```

### Running a Specific Day

```bash
cargo run <day_number>
```

For example, to run Day 1:

```bash
cargo run 1
```

### Running with Release Optimizations

```bash
cargo run --release <day_number>
```

## Adding Your Input

Replace the placeholder content in `inputs/dayXX.txt` with your actual puzzle input from the Advent of Code website.

## Implementing Solutions

Each day's solution is in its own file (`src/dayXX.rs`). Each file contains:

- `solve()` - The main function that reads input and calls both parts
- `solve_part1(input)` - Implementation for Part 1
- `solve_part2(input)` - Implementation for Part 2

The `utils` module provides helper functions for reading input:

- `read_input(day)` - Returns the input as a single string
- `read_lines(day)` - Returns the input as a vector of strings (one per line)
- `read_parsed::<T>(day)` - Returns the input parsed as a vector of type T
