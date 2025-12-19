# Advent of Code 2025 - Rust

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Project Structure

```
2025/
├── Cargo.toml              # Project configuration
├── inputs/                 # Input files for each day
│   ├── day01.txt          # Actual puzzle input
│   ├── day01_test.txt     # Example input from problem description
│   └── ...
└── src/
    ├── lib.rs             # Shared utilities (read_input, etc.)
    ├── bin/               # Executable for each day
    │   ├── day01.rs
    │   └── ...
    └── days/              # Solution logic and tests
        ├── mod.rs
        ├── day01.rs       # Part 1 & 2 + tests
        └── ...
```

## Running Solutions

Run a specific day:

```bash
cargo run --bin day01
cargo run --bin day02
# etc.
```

Run with release optimizations (faster):

```bash
cargo run --release --bin day01
```

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific day:

```bash
cargo test day01
```

Run tests for a specific day module:

```bash
cargo test --lib day01
```

## Adding a New Day

When you're ready to solve a new day, create three files:

1. **src/days/dayXX.rs** - Your solution logic with tests
2. **src/bin/dayXX.rs** - Binary to run the solution
3. **inputs/dayXX.txt** - Your puzzle input
4. **inputs/dayXX_test.txt** (optional) - Example input

Then update `src/days/mod.rs` to include the new module.

### Quick Template

Use the `new_day.sh` script to generate these files automatically:

```bash
./new_day.sh 2
```
