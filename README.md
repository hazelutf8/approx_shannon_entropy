# Approximate Shannon Entropy

A rust library to calculate the approximate Shannon entropy of a slice.

Usable on no_std due to use of approximate natural log from [micromath](https://github.com/tarcieri/micromath).

# Usage

Add this to your Cargo.toml
```
[dependencies]
shannon-entropy = "0.1.0"
```

# Examples

```
$ cargo run --example three_bits
Shannon Entropy (approximate bits per byte): 1
```

```
$ cargo build --example stdin_entropy
$ echo A | ./target/debug/examples/stdin_entropy
```
