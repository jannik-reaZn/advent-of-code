# Advent of Code 2025 - Rust

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Running Solutions

Run a specific day:

```bash
cargo run --bin day0X
# etc.
```

Run with release optimizations (faster):

```bash
cargo run --release --bin day0X
```

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific day:

```bash
cargo test day0X
```

Run tests for a specific day module:

```bash
cargo test --lib day0X
```

### Quick Template

Use the `new_day.sh` script to generate these files automatically:

```bash
./new_day.sh X
```
