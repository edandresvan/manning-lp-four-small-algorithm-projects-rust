# Project 2: Problem-Solving with Recursion

## Milestone 6: N-Queens Redux

Write a recursive function with backtracking to solve the N queens problem. 


## How to execute

```shell
$ cargo run
```

## Results

There are four strategies to choose from:

* place_queens_1: exhaustive search
* place_queens_2: queen counting
* place_queens_3: attacking squares
* place_queens_4: queens redux

```rust
fn place_queens_4()
```

```text
Strategy: Attacks
Board: Rows: 8 Columns 8
Time: 47.95578ms
Placing Queens was successfully!
• • • • • • • Q 
• • • Q • • • • 
Q • • • • • • • 
• • Q • • • • • 
• • • • • Q • • 
• Q • • • • • • 
• • • • • • Q • 
• • • • Q • • • 
```

```text
Strategy: Redux
Board: Rows: 8 Columns 8
Time: 365.443µs
Placing Queens was successfully!
Q • • • • • • • 
• • • • • • Q • 
• • • • Q • • • 
• • • • • • • Q 
• Q • • • • • • 
• • • Q • • • • 
• • • • • Q • • 
• • Q • • • • •
```

```text
Strategy: Attacks
Board: Rows: 10 Columns 10
Time: 2.866913603s
Placing Queens was successfully!
• • • • • • • • • Q 
• • • • • • • Q • • 
• • • • Q • • • • • 
• • Q • • • • • • • 
Q • • • • • • • • • 
• • • • • Q • • • • 
• Q • • • • • • • • 
• • • • • • • • Q • 
• • • • • • Q • • • 
• • • Q • • • • • • 
```


```text
Strategy: Redux
Board: Rows: 10 Columns 10
Time: 433.423µs
Placing Queens was successfully!
Q • • • • • • • • • 
• • • • • • • Q • • 
• Q • • • • • • • • 
• • • • • • • • Q • 
• • • • • Q • • • • 
• • Q • • • • • • • 
• • • • • • • • • Q 
• • • Q • • • • • • 
• • • • • • Q • • • 
• • • • Q • • • • • 
```

```text
Strategy: Redux
Board: Rows: 12 Columns 12
Time: 1.522749ms
Placing Queens was successfully!
Q • • • • • • • • • • • 
• • • • • • • • Q • • • 
• Q • • • • • • • • • • 
• • • • • • • • • • • Q 
• • Q • • • • • • • • • 
• • • • • • Q • • • • • 
• • • • • • • • • Q • • 
• • • Q • • • • • • • • 
• • • • • • • • • • Q • 
• • • • Q • • • • • • • 
• • • • • • • Q • • • • 
• • • • • Q • • • • • • 
```

```text
Strategy: Redux
Board: Rows: 16 Columns 16
Time: 120.65935ms
Placing Queens was successfully!
Q • • • • • • • • • • • • • • • 
• • • Q • • • • • • • • • • • • 
• Q • • • • • • • • • • • • • • 
• • • • • • • • • • • • Q • • • 
• • Q • • • • • • • • • • • • • 
• • • • • • • • • Q • • • • • • 
• • • • • • • • • • • Q • • • • 
• • • • • • • • • • • • • • Q • 
• • • • • Q • • • • • • • • • • 
• • • • • • • • • • • • • • • Q 
• • • • • • • • • • • • • Q • • 
• • • • • • • Q • • • • • • • • 
• • • • Q • • • • • • • • • • • 
• • • • • • Q • • • • • • • • • 
• • • • • • • • Q • • • • • • • 
• • • • • • • • • • Q • • • • • 
```

```text
Strategy: Redux
Board: Rows: 20 Columns 20
Time: 3.727441119s
Placing Queens was successfully!
Q • • • • • • • • • • • • • • • • • • • 
• • • Q • • • • • • • • • • • • • • • • 
• Q • • • • • • • • • • • • • • • • • • 
• • • • Q • • • • • • • • • • • • • • • 
• • Q • • • • • • • • • • • • • • • • • 
• • • • • • • • • • • • • • • • • • Q • 
• • • • • • • • • • • • • • • • Q • • • 
• • • • • • • • • • • • • • Q • • • • • 
• • • • • • • • • • • Q • • • • • • • • 
• • • • • • • • • • • • • • • Q • • • • 
• • • • • • • • • • • • • • • • • • • Q 
• • • • • • • Q • • • • • • • • • • • • 
• • • • • Q • • • • • • • • • • • • • • 
• • • • • • • • • • • • • • • • • Q • • 
• • • • • • Q • • • • • • • • • • • • • 
• • • • • • • • • • • • Q • • • • • • • 
• • • • • • • • • • Q • • • • • • • • • 
• • • • • • • • Q • • • • • • • • • • • 
• • • • • • • • • • • • • Q • • • • • • 
• • • • • • • • • Q • • • • • • • • • • 
```
