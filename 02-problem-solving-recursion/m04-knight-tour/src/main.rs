use std::{
  fmt::Display,
  time::{Duration, Instant},
};

/// Number of rows of the board.
const NUM_ROWS: usize = 8;
/// Number of columns of the boards.
const NUM_COLS: usize = 8;
/// Signed number of rows of the board.
const INUM_ROWS: i32 = NUM_ROWS as i32;
/// Signed number of columns of the board.
const INUM_COLS: i32 = NUM_COLS as i32;

const KNIGHT_TOUR_KIND: TourKind = TourKind::ClosedTour;

// Two-dimensional vector with the 8 possible pattern moves that the knight can make
// in a 'L' shape. I which each row :
// - the first element (index 0) is the horizontal move
// - the second element (index 1) is the vertical move
const OFFSETS: [[i32; 2]; 8] = [
  [-2, -1],
  [-1, -2],
  [2, -1],
  [1, -2],
  [-2, 1],
  [-1, 2],
  [2, 1],
  [1, 2],
];

/// Type of our of the knight.
#[allow(dead_code)]
#[derive(Debug)]
enum TourKind {
  /// An open tour.
  OpenTour,
  /// A closed tour.
  ClosedTour,
}

/// Value to represent a square that we have not visited.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(i32)]
enum SquareStatus {
  /// The square is visited with the indicated number step.
  Visited(i32),
  /// The square has not been visited. The default value is -1.
  Unvisited,
}

impl Display for SquareStatus {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    // Use explicitly the desired format by the user of the i32 value
    // Here, write!() does not preserve the user desired format
    // Source: https://stackoverflow.com/questions/72544241/how-to-implement-display-while-respecting-width-fill-and-alignment
    match *self {
      Self::Visited(n) => n.fmt(f),
      Self::Unvisited => i32::fmt(&-1, f),
    }
  }
}

/// Checks whether a solution for the knight's tour is found.
///
/// # Arguments
///
/// * `board`: Currrent board for moving the knight.
/// * `cur_row`: Knight's current row location.
/// * `cur_col`: Knight's current column location.
/// * `num_visited`: Number of squares that the knight has visited in its current tour including its current square.
///
/// # Returns
///
/// * `true`: A solution for knight's tour was found.
/// * `false`: No solution for knight's tour was found.
fn find_tour(
  board: &mut [[SquareStatus; NUM_COLS]; NUM_ROWS],
  cur_row: i32,
  cur_col: i32,
  num_visited: usize,
) -> bool {
  // Get the dimensions of the board.
  let num_rows = board.len();
  let num_cols = board[0].len();
  // Check if the knight has visited every square by comparing the number of visited
  // squares with the number of squares of the board
  if num_visited == num_rows * num_cols {
    return match KNIGHT_TOUR_KIND {
      // In an open tour, the final move does not need to be at the starting square.
      TourKind::OpenTour => true,
      // In an closed tour, the final move must be at the starting square.
      //    Iterate over the list of valid moves from the knight's current location and
      //    look for at least one of such next moves puts the knight's at the starting
      //    square that contains the very first move, that is 0.
      //    Use Rust any() function predicate.
      TourKind::ClosedTour => {
        get_valid_moves(*board, cur_row, cur_col)
          .iter()
          .any(|&possible_move| {
            board[possible_move.0 as usize][possible_move.1 as usize]
              == SquareStatus::Visited(0)
          })
      }
    };
  }

  // The knight has not visited every square so continue the search.
  for possible_move in get_valid_moves(*board, cur_row, cur_col) {
    board[possible_move.0 as usize][possible_move.1 as usize] =
      SquareStatus::Visited(num_visited as i32);
    if find_tour(board, possible_move.0, possible_move.1, num_visited + 1) {
      return true;
    }
    board[possible_move.0 as usize][possible_move.1 as usize] = SquareStatus::Unvisited;
  }
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
  println!("Kind of Tour: {:?}", KNIGHT_TOUR_KIND);
  println!("Time: {:?}", duration);
  if successful {
    println!("A knight's tour was successfully found!");
  } else {
    println!("Could not find a tour.");
  }

  dump_board(board);
}

/// Displays the board on screen.
/// 
/// # Arguments
/// 
/// * `board`: The board to be displayed on screen.
fn dump_board(board: &[[SquareStatus; NUM_COLS]; NUM_ROWS]) {
  for r in 0..NUM_ROWS {
    for c in 0..NUM_COLS {
      print!("{:02} ", board[r][c]);
    }
    println!();
  }
  println!();
}

/// Checks if the given row and column values represents a valid move inside the
/// current board status.
///
/// # Arguments
///
/// * `board`: Current board.
/// * `row`: The row index from which you want to know if the movement is valid.
/// * `column`: The column index from which you want to know if the movement is valid.
///
/// # Returns
///
/// * `true`: If the move is inside the board and the square has not been visited before.
/// * `false: Otherwise.
fn is_valid_move(
  board: [[SquareStatus; NUM_COLS]; NUM_ROWS],
  row: i32,
  column: i32,
) -> bool {
  // Will the knight's move be inside the board?

  // Will the knight's move be outside the rows of the board?
  if row < 0 || row >= INUM_ROWS {
    return false;
  }

  // Will the knight's move be outside the columns of the board?
  if column < 0 || column >= INUM_COLS {
    return false;
  }

  // Will the knight's move be on already visited square?
  match board[row as usize][column as usize] {
    // The square has already been visited.
    SquareStatus::Visited(_) => false,
    // All right, the knight's move will be inside the board and on unvisited square
    SquareStatus::Unvisited => true,
  }
}

/// Gets a list of valid moves that the knight can make given the status of the board,
/// and a location of row and column.
///
/// # Arguments
///
/// * `board`: Current board.
/// * `cur_row`: Current knight's row location.
/// * `cor_col`: Current knight's column location.
///
/// # Returns
///
/// A list of valid moves that the knight can make. If the knight cannot make any valid
/// move, then the returned list is empty-
fn get_valid_moves(
  board: [[SquareStatus; NUM_COLS]; NUM_ROWS],
  cur_row: i32,
  cur_col: i32,
) -> Vec<(i32, i32)> {
  let mut valid_moves: Vec<(i32, i32)> = Vec::with_capacity(8);

  // The knight has 8 possible moves in 'L' shape. For each possible 'L' shape try if it
  // is a valid move in the current board status
  for i in 0..8 {
    let new_row = cur_row + OFFSETS[i][0];
    let new_col = cur_col + OFFSETS[i][1];

    if is_valid_move(board, new_row, new_col) {
      valid_moves.push((new_row, new_col));
    }
  }
  valid_moves
}

fn main() {
  // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
  let mut board = [[SquareStatus::Unvisited; NUM_COLS]; NUM_ROWS];

  // Start at board[0][0].

  board[0][0] = SquareStatus::Visited(0);

  // Try to find a tour.
  let start = Instant::now();
  let success = find_tour(&mut board, 0, 0, 1);
  let duration = start.elapsed();

  show_results(&board, duration, success);
}
