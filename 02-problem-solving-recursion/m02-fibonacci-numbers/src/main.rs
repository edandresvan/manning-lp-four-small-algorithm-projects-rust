/// Calculates the last fibonacci number of the given number.
///
/// # Arguments
///
/// * `number`: Number to calculate its fibonacci number.
///
/// # Returns
///
/// The last fibonacci number calculated for the given number.
fn fibonacci(number: i64) -> i64 {
  match number {
    // 0 and 1 are the base case
    0 => 0,
    1 => 1,
    // number >= 2 is the recursive case
    number if number >= 2 => fibonacci(number - 1) + fibonacci(number - 2),
    n => panic!(
      "fibonacci is not allowed for negative numbers such as {}",
      n
    ),
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fibonacci_zero_one() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
  }

  #[test]
  fn test_fibonnaci_upto_ten() {
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
    assert_eq!(fibonacci(8), 21);
    assert_eq!(fibonacci(9), 34);
    assert_eq!(fibonacci(10), 55);
  }

  #[test]
  fn test_fibonnaci_more_numbers() {
    assert_eq!(fibonacci(15), 610);
    assert_eq!(fibonacci(20), 6765);
    assert_eq!(fibonacci(25), 75025);
    assert_eq!(fibonacci(30), 832040);
    assert_eq!(fibonacci(35), 9227465);
    assert_eq!(fibonacci(40), 102334155);
  }

  #[test]
  #[should_panic(expected = "fibonacci is not allowed for negative numbers such as ")]
  fn test_fibonacci_not_for_negative_numbers() {
    fibonacci(-3);
  }
}
