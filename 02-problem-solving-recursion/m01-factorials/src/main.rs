/// Calculates the factorial of the given number.
///
/// # Arguments
///
/// * `number`: Number to calculate the factorial.
///
/// # Returns
///
/// The calculated factorial of the given number.
fn factorial(number: i64) -> i64 {
  // Base case
  if number == 0 {
    1
  }
  // Recursive case
  else {
    number * factorial(number - 1)
  }
}

fn main() {
  for n in 0..22 {
    println!("{}! = {}", n, factorial(n));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial_zero() {
    assert_eq!(factorial(0), 1);
  }

  #[test]
  fn test_factorial_one() {
    assert_eq!(factorial(1), 1);
  }

  #[test]
  fn test_factorional_twenty() {
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(6), 720);
    assert_eq!(factorial(7), 5040);
    assert_eq!(factorial(8), 40320);
    assert_eq!(factorial(9), 362880);
    assert_eq!(factorial(10), 3628800);
    assert_eq!(factorial(11), 39916800);
    assert_eq!(factorial(12), 479001600);
    assert_eq!(factorial(13), 6227020800);
    assert_eq!(factorial(14), 87178291200);
    assert_eq!(factorial(15), 1307674368000);
    assert_eq!(factorial(16), 20922789888000);
    assert_eq!(factorial(17), 355687428096000);
    assert_eq!(factorial(18), 6402373705728000);
    assert_eq!(factorial(19), 121645100408832000);
    assert_eq!(factorial(20), 2432902008176640000);
  }

  #[test]
  #[should_panic(expected = "attempt to multiply with overflow")]
  fn test_factorial_21_panics() {
    factorial(21);
  }
}
