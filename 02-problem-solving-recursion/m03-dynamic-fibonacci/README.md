# Project 2: Problem-Solving with Recursion

## Milestone 3: Dynamic Fibonacci Numbers

Calculate the Fibonacci number of a given integer number n using a function with dynamic programming:

```ada
F(0) = 0
F(1) = 1
F(n) = F(n-1) + F(n - 2)
```

```rust
fn fibonacci_on_the_fly(
  values: &mut Vec<i64>,
  number: i64,
) -> i64 {}

fn fibonacci_prefill_vector() -> Vec<i64> {}

fn fibonacci_bottom_up(number: i64) -> i64 {}
```

## How to execute

```shell
$ cargo run
```