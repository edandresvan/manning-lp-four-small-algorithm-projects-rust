#![allow(clippy::needless_range_loop)]

use std::{
  io::{self, Write},
  time::{SystemTime, UNIX_EPOCH},
};

/// Sorts the given vector in place using the bubble sort algorithm.
/// 
/// # Arguments
/// 
/// * `vec`: Mutable vector containing numbers to be sorted.
/// 
/// # Remarks
/// 
/// After the execution of this function, the elements in the given vector 
/// will be sorted.
fn bubble_sort(vec: &mut Vec<i32>) {
  // Implement the bubble algorithm shown in Wikipedia as required:
  // Source: https://en.wikipedia.org/wiki/Bubble_sort#Pseudocode_implementation
  loop {
    let mut swapped: bool = false;
    for index in 1..vec.len() {
      if vec[index - 1] > vec[index] {
        vec.swap(index - 1, index);
        swapped = true
      }
    }
    if !swapped {
      break;
    }
  }
}

/// Verifies that the given vector is sorted.
///
/// # Arguments
///
/// * `vec`: The vector of numbers to be sorted.
///
/// # Returns
///
/// * `true`: if the given vector is already sorted.
/// * `fallse`: otherwise.
fn check_sorted(vec: &Vec<i32>) -> bool {
  // Iterate every element by index from the second element
  for index in 1..vec.len() {
    // The predecessor element should not be larger than the current element.
    // If this occurs the method should return false immediately
    if vec[index - 1] > vec[index] {
      return false;
    }
  }
  true
}

/// Represents a pseudorandom number generator that uses a
/// a linear congruential generator (LCG).
struct Prng {
  seed: u32,
}

impl Prng {
  fn new() -> Self {
    let mut prng = Self { seed: 0 };
    prng.randomize();
    prng
  }

  fn randomize(&mut self) {
    let millis = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis();
    self.seed = millis as u32;
  }

  /// Returns a pseudorandom value in the range [0, 2147483647].
  fn next_u32(&mut self) -> u32 {
    self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
    self.seed %= 1 << 31;
    self.seed
  }

  /// Returns a pseudorandom value in the range [0.0, 1.0).
  fn next_f64(&mut self) -> f64 {
    let f = self.next_u32() as f64;
    f / (2147483647.0 + 1.0)
  }

  /// Returns a pseudorandom value in the range [min, max).
  fn next_i32(
    &mut self,
    min: i32,
    max: i32,
  ) -> i32 {
    let range = (max - min) as f64;
    let result = min as f64 + range * self.next_f64();
    result as i32
  }
}

/// Prompts the user for an integer number.
///
/// # Arguments
///
/// * `prompt`: Message to show to the user.
///
/// # Returns
///
/// An integer entered by the user.
fn get_i32(prompt: &str) -> i32 {
  print!("{prompt}");
  io::stdout().flush().unwrap();

  let mut str_value = String::new();
  io::stdin()
    .read_line(&mut str_value)
    .expect("Error reading input");

  str_value
    .trim()
    .parse::<i32>()
    .expect("Error parsing integer")
}

/// Makes a vector of random i32 values in the range [0 and max).
fn make_random_vec(
  num_items: i32,
  max: i32,
) -> Vec<i32> {
  // Prepare a Prng.
  let mut prng = Prng::new();

  let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
  for _ in 0..num_items {
    vec.push(prng.next_i32(0, max));
  }
  vec
}

// Print at most num_items items.
fn print_vec(
  vec: &Vec<i32>,
  num_items: i32,
) {
  let mut max = vec.len();
  if max > num_items as usize {
    max = num_items as usize;
  }

  let mut string = String::new();
  string.push('[');

  if max > 0usize {
    string.push_str(&vec[0].to_string());
  }

  for i in 1usize..max {
    string.push(' ');
    string.push_str(&vec[i].to_string());
  }
  string.push(']');
  println!("{string}");
}

fn main() {
  // Prompt the user for the number of items and the maximum item value
  let num_items: i32 = get_i32("Number of items: ");
  let max: i32 = get_i32("Maximum item number: ");

  // Create a vector containing random values.
  let mut numbers: Vec<i32> = make_random_vec(num_items, max);

  // Sort the vector and shows its state before and after sorting it
  print_vec(&numbers, 0);
  bubble_sort(&mut numbers);
  print_vec(&numbers, 30);

  // Verify if the numbers are sorted
  println!("Is the vector sorted:? {}", check_sorted(&numbers));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bubble_sort() {
    let mut numbers: Vec<i32> = vec![
      3, 8, 1, 7, 4, 14, 2, 18, 5, 18, 3, 12, 13, 0, 11, 13, 10, 6, 6, 0, 16, 9, 13, 12,
      17, 4, 8, 13, 16, 12,
    ];
    let sorted_numbers: Vec<i32> = vec![
      0, 0, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 11, 12, 12, 12, 13, 13, 13, 13,
      14, 16, 16, 17, 18, 18,
    ];
    bubble_sort(&mut numbers);
    assert_eq!(numbers, sorted_numbers);
  }

  #[test]
  fn test_check_sorted_success() {
    let sorted_numbers: Vec<i32> = vec![
      0, 0, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 10, 11, 12, 12, 12, 13, 13, 13, 13,
      14, 16, 16, 17, 18, 18,
    ];
    assert!(check_sorted(&sorted_numbers));
  }

  #[test]
  fn test_check_sorted_failure() {
    let unsorted_numbers: Vec<i32> = vec![
      3, 8, 1, 7, 4, 14, 2, 18, 5, 18, 3, 12, 13, 0, 11, 13, 10, 6, 6, 0, 16, 9, 13, 12,
      17, 4, 8, 13, 16, 12,
    ];
    assert!(!check_sorted(&unsorted_numbers));
  }
}
