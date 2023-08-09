#[cfg(test)]
#[path = "main.rs"]
mod main_tests {
  use super::super::*;

  #[test]
  fn test_fibonacci_on_the_fly_positive() {
    // Create a vector for fill-on-the-fly.
    let mut values: Vec<i64> = vec![0, 1];
    // Get Fibonacci number in order
    assert_eq!(fibonacci_on_the_fly(&mut values, 0), 0);
    assert_eq!(fibonacci_on_the_fly(&mut values, 1), 1);
    assert_eq!(values, vec![0, 1]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 2), 1);
    assert_eq!(values, vec![0, 1, 1]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 3), 2);
    assert_eq!(values, vec![0, 1, 1, 2]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 4), 3);
    assert_eq!(values, vec![0, 1, 1, 2, 3]);
    // Get Fibonacci number in random
    assert_eq!(fibonacci_on_the_fly(&mut values, 9), 34);
    assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 5), 5);
    assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 7), 13);
    assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 8), 21);
    assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(fibonacci_on_the_fly(&mut values, 6), 8);
    assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);

    assert_eq!(fibonacci_on_the_fly(&mut values, 10), 55);
    assert_eq!(fibonacci_on_the_fly(&mut values, 15), 610);
    assert_eq!(fibonacci_on_the_fly(&mut values, 20), 6765);
    assert_eq!(fibonacci_on_the_fly(&mut values, 25), 75025);
    assert_eq!(fibonacci_on_the_fly(&mut values, 30), 832040);
    assert_eq!(fibonacci_on_the_fly(&mut values, 35), 9227465);
    assert_eq!(fibonacci_on_the_fly(&mut values, 40), 102334155);
    assert_eq!(fibonacci_on_the_fly(&mut values, 45), 1134903170);

    assert_eq!(fibonacci_on_the_fly(&mut values, 34), 5702887);
    assert_eq!(fibonacci_on_the_fly(&mut values, 59), 956722026041);
    assert_eq!(fibonacci_on_the_fly(&mut values, 76), 3416454622906707);
    assert_eq!(fibonacci_on_the_fly(&mut values, 92), 7540113804746346429);
  }

  #[test]
  #[should_panic(expected = "Fibonacci is not allowed for negative numbers such as ")]
  fn test_fibonacci_on_the_fly_not_for_negative_numbers() {
    // Create a vector for fill-on-the-fly.
    let mut values: Vec<i64> = vec![0, 1];
    fibonacci_on_the_fly(&mut values, -3);
  }

  #[test]
  fn test_fibonacci_prefill_vector() {
    // Test that there is 93 Fibonacci values, that is F(0) up to F(92).
    assert_eq!(fibonacci_prefill_vector().len(), 93);
    // Test the Fibonacci values up to F(20).
    assert_eq!(
      fibonacci_prefill_vector()[0..=20],
      vec![
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584,
        4181, 6765
      ]
    );
    // Test the last three Fibonacci numbers F(90), F(91), F(92)
    assert_eq!(
      fibonacci_prefill_vector()[(93 - 3)..],
      [
        2880067194370816120,
        4660046610375530309,
        7540113804746346429
      ]
    );
    // Test some Fibonacci numbers F(34), F(59), F(76), F(92)
    assert_eq!(fibonacci_prefill_vector()[34], 5702887);
    assert_eq!(fibonacci_prefill_vector()[59], 956722026041);
    assert_eq!(fibonacci_prefill_vector()[76], 3416454622906707);
    assert_eq!(fibonacci_prefill_vector()[92], 7540113804746346429);
  }

  #[test]
  fn test_fibonacci_bottom_up() {
    assert_eq!(fibonacci_bottom_up(0), 0);
    assert_eq!(fibonacci_bottom_up(1), 1);

    assert_eq!(fibonacci_bottom_up(2), 1);
    assert_eq!(fibonacci_bottom_up(3), 2);
    assert_eq!(fibonacci_bottom_up(4), 3);
    assert_eq!(fibonacci_bottom_up(5), 5);
    assert_eq!(fibonacci_bottom_up(6), 8);
    assert_eq!(fibonacci_bottom_up(7), 13);
    assert_eq!(fibonacci_bottom_up(8), 21);
    assert_eq!(fibonacci_bottom_up(9), 34);
    assert_eq!(fibonacci_bottom_up(10), 55);

    assert_eq!(fibonacci_bottom_up(15), 610);
    assert_eq!(fibonacci_bottom_up(20), 6765);
    assert_eq!(fibonacci_bottom_up(25), 75025);
    assert_eq!(fibonacci_bottom_up(30), 832040);
    assert_eq!(fibonacci_bottom_up(35), 9227465);
    assert_eq!(fibonacci_bottom_up(40), 102334155);

    assert_eq!(fibonacci_bottom_up(34), 5702887);
    assert_eq!(fibonacci_bottom_up(59), 956722026041);
    assert_eq!(fibonacci_bottom_up(76), 3416454622906707);
    assert_eq!(fibonacci_bottom_up(92), 7540113804746346429);
  }

  #[test]
  #[should_panic(expected = "Fibonacci is not allowed for negative numbers such as ")]
  fn test_fibonacci_bottom_up_not_for_negative_numbers() {
    fibonacci_bottom_up(-3);
  }
}
