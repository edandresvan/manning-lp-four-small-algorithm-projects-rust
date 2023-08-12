# Project 2: Problem-Solving with Recursion

## Milestone 5: N-Queens

Write a recursive function with backtracking to solve the N queens problem. 


## How to execute

```shell
$ cargo run
```

## Results

There are three strategies to choose from suitable for different board sizes:

* place_queens_1: exhaustive search
* place_queens_2: queen counting
* place_queens_3: attacking squares

### N-Qeens with exhaustive search

```rust
fn place_queens_1()
```

```text
Board: Rows: 4 Columns 4
Time: 1.00827ms
Placing Queens was successfully!
• • Q • 
Q • • • 
• • • Q 
• Q •  
```


```text
Board: Rows: 5 Columns 5
Time: 110.705856ms
Placing Queens was successfully!
• • • • Q 
• • Q • • 
Q • • • • 
• • • Q • 
• Q • • • 
```


```text
Board: Rows: 6 Columns 6
Time: 228.831715196s
Placing Queens was successfully!
• • • • Q • 
• • Q • • • 
Q • • • • • 
• • • • • Q 
• • • Q • • 
• Q • • • • 
```

### N-Qeens with counting

```rust
fn place_queens_2()
```

```text
Board: Rows: 6 Columns 6
Time: 149.070657ms
Placing Queens was successfully!
• • • • Q • 
• • Q • • • 
Q • • • • • 
• • • • • Q 
• • • Q • • 
• Q • • • • 
```

```text
Board: Rows: 7 Columns 7
Time: 5.843440544s
Placing Queens was successfully!
• • • • • • Q 
• • • • Q • • 
• • Q • • • • 
Q • • • • • • 
• • • • • Q • 
• • • Q • • • 
• Q • • • • • 
```

### N-Qeens with attacks

```rust
fn place_queens_3()
```

```text
Board: Rows: 8 Columns 8
Time: 40.963965ms
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
Board: Rows: 9 Columns 9
Time: 313.263372ms
Placing Queens was successfully!
• • • • • • • • Q 
• • • • • • Q • • 
• • • Q • • • • • 
• Q • • • • • • • 
• • • • • • • Q • 
• • • • • Q • • • 
Q • • • • • • • • 
• • Q • • • • • • 
• • • • Q • • • • 
```

```text
Board: Rows: 10 Columns 10
Time: 2.751177627s
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