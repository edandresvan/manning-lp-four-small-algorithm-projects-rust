# Project 2: Problem-Solving with Recursion

## Milestone 4: Knight's Tour

Find a path that a knight could use to visit every square on a chessboard without visiting any square twice. There are two kind of tours:

* Open Tour: the knight can start and end on any square.
* Closed Tour: the knight must finish so it could move back to its starting position in one more move to make a closed loop.

The app uses a recursive function with backtracking to solve the knight’s tour problem.


## How to execute

```shell
$ cargo run
```


## Results

The following are some of the succesul knight's tours given by this program.

### Open Knight's Tour

```text
Board: Rows: 4 Columns 4
Kind of Tour: OpenTour
Time: 696.388µs
Could not find a tour.
00 -1 -1 -1 
-1 -1 -1 -1 
-1 -1 -1 -1 
-1 -1 -1 -1
```

```text
Board: Rows: 5 Columns 5
Kind of Tour: OpenTour
Time: 1.878648ms
A knight's tour was successly found!
00 09 18 13 22 
17 04 21 08 19 
10 01 12 23 14 
05 16 03 20 07 
02 11 06 15 24
```

```text
Board: Rows: 6 Columns 6
Kind of Tour: OpenTour
Time: 7.245956326s
A knight's tour was successly found!
00 11 20 33 30 35 
21 04 13 10 19 32 
12 01 22 31 34 29 
05 14 03 26 09 18 
02 23 16 07 28 25 
15 06 27 24 17 08
```

```text
Board: Rows: 7 Columns 7
Kind of Tour: OpenTour
Time: 133.14929291s
A knight's tour was successly found!
00 07 12 43 14 17 48 
11 04 09 16 47 42 45 
08 01 06 13 44 15 18 
05 10 03 22 31 46 41 
02 25 28 37 34 19 32 
27 38 23 30 21 40 35 
24 29 26 39 36 33 20 
```

### Closed Knight's Tour

```text
Board: Rows: 4 Columns 4
Kind of Tour: ClosedTour
Time: 637.332µs
Could not find a tour.
00 -1 -1 -1 
-1 -1 -1 -1 
-1 -1 -1 -1 
-1 -1 -1 -1
```

```text
Board: Rows: 5 Columns 5
Kind of Tour: ClosedTour
Time: 447.95745ms
Could not find a tour.
00 -1 -1 -1 -1 
-1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 
-1 -1 -1 -1 -1
```

```text
Board: Rows: 5 Columns 7
Kind of Tour: ClosedTour
Time: 1721.192406683s
Could not find a tour.
00 -1 -1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 -1 -1 
-1 -1 -1 -1 -1 -1 -1
```

Note: Greater board dimension takes a long time to process.

