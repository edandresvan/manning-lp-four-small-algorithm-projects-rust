use std::{
  fmt::Display,
  time::{Duration, Instant},
};

/// Number of rows of the board.
const NUM_ROWS: usize = 5;
/// Number of columns of the boards.
const NUM_COLS: usize = NUM_ROWS;

/// Represents the status of a square in a board.
#[derive(Copy, Clone, PartialEq)]
enum SquareStatus {
  /// The square holds a queen.
  Queen,
  /// The square is empty.
  Empty,
}

impl Display for SquareStatus {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    // Use the format desired by the user.
    // Source: https://stackoverflow.com/questions/72544241/how-to-implement-display-while-respecting-width-fill-and-alignment
    match self {
      Self::Queen => "Q".fmt(f),
      Self::Empty => "•".fmt(f),
    }
  }
}

/// Determines whether the given specified series of squares contains at most one queen.
///
/// # Arguments
///
/// * `board`: Board to which the specified series belongs.
/// * `r0`: Row index of the first desired series of squares.
/// * `c0`: Column index of the first desired series of squares.
/// * `dr`: Increment of the row index when traversing this series.
/// * `dc`: Increment of the column index when traversing this series.
///
/// # Returns
///
/// * `true`: If there is zero or one queen in this series.
/// * `false`: If there are two or more queens in this series.
fn series_is_legal(
  board: &[[SquareStatus; NUM_COLS]; NUM_ROWS],
  r0: i32,
  c0: i32,
  dr: i32,
  dc: i32,
) -> bool {
  let mut has_queen = false;

  let mut current_row = r0;
  let mut current_column: i32 = c0;
  loop {
    if board[current_row as usize][current_column as usize] == SquareStatus::Queen {
      // If we already have a queen on this row,
      // then this board is not legal.
      if has_queen {
        return false;
      }

      // Remember that we have a queen on this row.
      has_queen = true;
    }

    // Move to the next square in the series.
    current_row += dr;
    current_column += dc;

    // If we fall off the board, then the series is legal.
    if current_row >= NUM_ROWS as i32
      || current_column >= NUM_COLS as i32
      || current_row < 0
      || current_column < 0
    {
      return true;
    }
  }
}

/// Determines whether the board is legal by checking series in rows, columns and diagonals.
///
/// # Arguments
///
/// * `board`: Board that you want to determine if it is legal.
///
/// # Returns
///
/// * `true`: If the entire board is legal (every series is legal).
/// * `false`: Otherwise.
fn board_is_legal(board: &[[SquareStatus; NUM_COLS]; NUM_ROWS]) -> bool {
  // See if each row is legal.
  // Increment the row index
  for r in 0..NUM_ROWS as i32 {
    if !series_is_legal(board, r, 0, 0, 1) {
      return false;
    }
  }

  // See if each column is legal.
  // Increment the column index
  for c in 0..NUM_COLS as i32 {
    if !series_is_legal(board, 0, c, 1, 0) {
      return false;
    }
  }

  // See if diagonals down to the right are legal.
  // Increment row and column index by the same amount
  for r in 0..NUM_ROWS as i32 {
    if !series_is_legal(board, r, 0, 1, 1) {
      return false;
    }
  }
  for c in 0..NUM_COLS as i32 {
    if !series_is_legal(board, 0, c, 1, 1) {
      return false;
    }
  }

  // See if diagonals down to the left are legal.
  // Decrement row and column index by the same amount
  for r in 0..NUM_ROWS as i32 {
    if !series_is_legal(board, r, NUM_ROWS as i32 - 1, 1, -1) {
      return false;
    }
  }
  for c in 0..NUM_COLS as i32 {
    if !series_is_legal(board, 0, c, 1, -1) {
      return false;
    }
  }

  // If we survived this long, then the board is legal.
  true
}

/// Determines if the board is legal and with a solution.
///
/// # Arguments
///
/// * `board`: Board that you want to determine if it is legal and has a solution.
///
/// # Returns
///
/// * `true`: If the board is legal and has a solution.
/// * `false`: Otherwise.
fn board_is_a_solution(board: &[[SquareStatus; NUM_COLS]; NUM_ROWS]) -> bool {
  // See if it is legal.
  if !board_is_legal(board) {
    return false;
  }

  // See if the board contains exactly NUM_ROWS queens.
  let mut num_queens = 0;
  for r in 0..NUM_ROWS {
    for c in 0..NUM_COLS {
      if board[r][c] == SquareStatus::Queen {
        num_queens += 1;
      }
    }
  }

  num_queens == NUM_ROWS
}

/// Tries to the place a queen at the specified location.///
///
/// # Arguments
///
/// * `board`: Current board arrangement with  previously positioned queens.
/// * `r`: Row of the location to examine.
/// * `c`: Column of the location to examine.
///
/// # Returns
///
/// * `true`: If a legal board is found.
/// * `false`: Otherwise.
fn place_queens_1(
  board: &mut [[SquareStatus; NUM_COLS]; NUM_ROWS],
  r: i32,
  c: i32,
) -> bool {
  // Check if every position has been examined and now it is off the board.
  if r >= NUM_ROWS as i32 {
    // So, the possible queens has been positioned, so check if it is a valid solution
    return board_is_a_solution(board);
  }

  // It is inside the board, 

  // Find the next square location moving along the current row
  let mut next_row: i32 = r;
  let mut next_column: i32 = c + 1; // advance trought the columns of the row
  
  // If the column index falls of the board, move to the beginning of the next row
  if next_column >= NUM_COLS as i32 {
    next_row += 1;
    next_column = 0;
  }

  // See what happens if we place a queen in the next square location
  if place_queens_1(board, next_row, next_column) {
    return true;
  }

  // See what happens if we do place a queen in the given square location
  board[r as usize][c as usize] = SquareStatus::Queen;

  // See what happens if we place a queen in the next square location
  if place_queens_1(board, next_row, next_column) {
    return true;
  }

  // Here, definitely the recursive calls cannot find a solution for the given location.
  // So, backtrack by marking the current square empty.
  board[r as usize][c as usize] = SquareStatus::Empty;

  // No solution. Return false.
  false
}

/// Shows the results to the user.
///
/// # Arguments
///
/// * `board`: Current board.
/// * `duration`: Time duration that took trying to find a solution.
/// * `successful`: Indicates whether or not a solution was found.
fn show_results(
  board: &[[SquareStatus; NUM_COLS]; NUM_ROWS],
  duration: Duration,
  successful: bool,
) {
  println!("Board: Rows: {} Columns {}", NUM_ROWS, NUM_COLS);
  println!("Time: {:?}", duration);
  if successful {
    println!("Placing Queens was successfully!");
  } else {
    println!("Could not place the queens.");
  }

  dump_board(board);
}

/// Displays the board on screen.
///
/// # Arguments
///
/// * `board`: The board to be displayed on screen.
fn dump_board(board: &[[SquareStatus; NUM_COLS]; NUM_ROWS]) {
  for r in 0..board.len() {
    for c in 0..board[0].len() {
      print!("{:^2}", board[r][c]);
    }
    println!();
  }
  println!();
}

fn main() {
  // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
  let mut board = [[SquareStatus::Empty; NUM_COLS]; NUM_ROWS];

  let start = Instant::now();

  // Here, there are three strategies to choose from: 
  // - place_queens_1: exhaustive search
  // - place_queens_2: queen counting
  // - place_queens_3: attacking squares
  let success = place_queens_1(&mut board, 0, 0);
  // let success = place_queens_2(&mut board, 0, 0, 0);
  //let success = place_queens_3(& mut board);

  let duration = start.elapsed();

  show_results(&board, duration, success);
}

/* N-Queens problem with counting */

// Try placing a queen at position [r][c].
// Keep track of the number of queens placed.
// Return true if we find a legal board.
fn place_queens_2(board: &mut [[SquareStatus; NUM_COLS]; NUM_ROWS], mut num_placed: i32, r: i32, c: i32) -> bool {
  // See if we have placed all of the queens.
  if num_placed == NUM_ROWS as i32 {
      // See if this is a solution.
      return board_is_a_solution(board);
  }

  // See if we have examined the whole board.
  if r >= NUM_ROWS as i32 {
      // We have examined all of the squares but this is not a solution.
      return false;
  }

  // Find the next square.
  let mut next_r = r;
  let mut next_c = c + 1;
  if next_c >= NUM_ROWS as i32 {
      next_r += 1;
      next_c = 0;
  }

  // Leave no queen in this square and
  // recursively assign the next square.
  if place_queens_2(board, num_placed, next_r, next_c) {
      return true;
  }

  // Try placing a queen here and
  // recursively assigning the next square.
  board[r as usize][c as usize] = SquareStatus::Queen;
  num_placed += 1;
  if place_queens_2(board, num_placed, next_r, next_c) {
      return true;
  }

  // That didn't work so remove this queen.
  board[r as usize][c as usize] = SquareStatus::Empty;
  num_placed -= 1;

  // If we get here, then there is no solution from
  // the board position before this function call.
  // Return false to backtrack and try again farther up the call stack.
  false
}



/* N-Queens problem with attacks  */

// Set up and call place_queens_3.
fn place_queens_3(board: &mut [[SquareStatus; NUM_COLS]; NUM_ROWS]) -> bool {
  // Make the num_attacking array.
  // The value num_attacking[r as usize][c as usize] is the number
  // of queens that can attack square (r, c).
  let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

  // Call do_place_queens_3.
  let mut num_placed = 0;
  do_place_queens_3(board, num_placed, 0, 0, &mut num_attacking)
}

// Try placing a queen at position [r][c].
// Keep track of the number of queens placed.
// Keep running totals of the number of queens attacking a square.
// Return true if we find a legal board.
fn do_place_queens_3(board: &mut [[SquareStatus; NUM_COLS]; NUM_ROWS], mut num_placed: i32, r: i32, c: i32,
  num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS]) -> bool
{
  // See if we have placed all of the queens.
  if num_placed == NUM_ROWS as i32 {
      // See if this is a solution.
      return board_is_a_solution(board);
  }

  // See if we have examined the whole board.
  if r >= NUM_ROWS as i32 {
      // We have examined all of the squares but this is not a solution.
      return false;
  }

  // Find the next square.
  let mut next_r = r;
  let mut next_c = c + 1;
  if next_c >= NUM_ROWS as i32 {
      next_r += 1;
      next_c = 0;
  }

  // Leave no queen in this square and
  // recursively assign the next square.
  if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
      return true;
  }

  // See if we can place a queen at (r, c).
  if num_attacking[r as usize][c as usize] == 0 {
      // Try placing a queen here and
      // recursively assigning the next square.
      board[r as usize][c as usize] = SquareStatus::Queen;
      num_placed += 1;

      // Increment the attack counts for this queen.
      adjust_attack_counts(num_attacking, r, c, 1);

      if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
          return true;
      }
  
      // That didn't work so remove this queen.
      board[r as usize][c as usize] = SquareStatus::Empty;
      num_placed -= 1;
      adjust_attack_counts(num_attacking, r, c, -1);
  }

  // If we get here, then there is no solution from
  // the board position before this function call.
  // Return false to backtrack and try again farther up the call stack.
  false
}

// Add amount to the attack counts for this square.
fn adjust_attack_counts(num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS], r: i32, c: i32, amount: i32) {
  // Attacks in the same row.
  for i in 0..NUM_COLS {
      num_attacking[r as usize][i] += amount
  }

  // Attacks in the same column.
  for i in 0..NUM_ROWS {
      num_attacking[i][c as usize] += amount
  }

  // Attacks in the upper left to lower right diagonal.
  for i in -(NUM_ROWS as i32)..(NUM_ROWS as i32) {
      let test_r = r + i;
      let test_c = c + i;
      if test_r >= 0 && test_r < NUM_ROWS as i32 &&
         test_c >= 0 && test_c < NUM_ROWS as i32 {
              num_attacking[test_r as usize][test_c as usize] += amount;
         }
  }

  // Attacks in the upper right to lower left diagonal.
  for i in -(NUM_ROWS as i32)..(NUM_ROWS as i32) {
      let test_r = r + i;
      let test_c = c - i;
      if test_r >= 0 && test_r < NUM_ROWS as i32 &&
         test_c >= 0 && test_c < NUM_ROWS as i32 {
              num_attacking[test_r as usize][test_c as usize] += amount;
         }
  }
}