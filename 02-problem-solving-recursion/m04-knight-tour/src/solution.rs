// Knight's tour

use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 4;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [ 2, -1],
        [ 1, -2],
        [-2,  1],
        [-1,  2],
        [ 2,  1],
        [ 1,  2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

// Display the board.
fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            print!("{:<02} ", board[r][c]);
        }
        println!();
    }
    println!();
}

// Note: I tried making a legal_moves function that
// created a slice containing the legal moves from a square.
// Unfortunately creating and appending to all of those slices
// slowed the program *greatly*. It's faster to just Loop
// through the offsets each time you need legal moves.

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8],             // 8 possible moves, 2 coordinates each.
    cur_row: i32, cur_col: i32,
    num_visited: i32) -> bool
{
    // If num_visited == NUM_ROWS * NUM_COLS,
    // then we have visited every square.
    if num_visited == (INUM_ROWS * INUM_COLS) {
        // Do one of the following.
        if !REQUIRE_CLOSED_TOUR {
            // Accept any path that visits every square.
            return true;
        } else {
            // Check to see if this is a closed tour.
            for i in 0..8 {
                // Get the move.
                let r = cur_row + offsets[i][0];
                let c = cur_col + offsets[i][1];
        
                // See if this move is on the board.
                if r < 0 || r >= INUM_ROWS { continue; }
                if c < 0 || c >= INUM_COLS { continue; }
        
                // See if this is move 0.
                if board[r as usize][c as usize] == 0 {
                    return true;
                }
            }
            // We cannot move from here to the first
            // position so this is not a closed tour.
            return false;
        }
    }

    // Loop through the possible moves.
    for i in 0..8 {
        // Get the move.
        let r = cur_row + offsets[i][0];
        let c = cur_col + offsets[i][1];

        // See if this move is on the board.
        if r < 0 || r >= INUM_ROWS { continue; }
        if c < 0 || c >= INUM_COLS { continue; }

        // See if we have already visited this position.
        if board[r as usize][c as usize] != UNVISITED { continue; }

        // The move to [r][c] is viable.
        // Make this move.
        board[r as usize][c as usize] = num_visited;

        // Try to find the rest of a tour.
        // If we succeed, return true.
        if find_tour(board, offsets, r, c, num_visited + 1) { return true; }

        // We did not find a tour with this move. Unmake this move.
        board[r as usize][c as usize] = UNVISITED;
    }

    // None of the possible moves worked. Return false.
    return false;
}
