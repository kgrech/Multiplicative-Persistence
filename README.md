# Multiplicative-Persistence [![Build Status](https://travis-ci.com/kgrech/Multiplicative-Persistence.svg?branch=master)](https://travis-ci.com/kgrech/Multiplicative-Persistence)
Computes [multiplicative persistence](https://en.wikipedia.org/wiki/Persistence_of_a_number) of each number in the [0, 2^32-1] range and smallest the first number for each persistence value found.
Written in [Rust](https://www.rust-lang.org/).
View more on [YouTube](https://youtu.be/0xLlb08OMEI)

## Run
```
cargo run --release
```
or
```
cargo build --release
time ./target/release/mult_persistence
```
## Run tests
```
cargo test
```
