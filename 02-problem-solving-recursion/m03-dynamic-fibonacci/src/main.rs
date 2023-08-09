use std::io;
use std::io::Write;

/// Calculates the Fibonacci value of the given number and fills the given list
/// with the intermediary Fibonnaci values.
///
/// # Arguments
///
/// * `values`: Mutable list of precalculated Fibonacci values used to calculate the
/// requested Fibonacci number.
/// * `number`: Number to calculate its Fibonacci number.
///
/// # Returns
///
/// The last Fibonacci number calculated for the given number.
///
/// # Remarks
///
/// The given mutable list `values` is filled with ongoing Fibonacci values up to calculate
/// the requested Fibonacci number.
///
/// panics if `number` is negative.
fn fibonacci_on_the_fly(
  values: &mut Vec<i64>,
  number: i64,
) -> i64 {
  // Don't allow negative numbers.
  if number < 0 {
    panic!(
      "Fibonacci is not allowed for negative numbers such as {}",
      number
    )
  }

  // Search if the given number has its Fibonacci number already calculated in the list of
  // values
  match values.get(number as usize) {
    Some(value) => *value,
    None => {
      // Calculate the Fibonacci number using recursion.
      let value: i64 = fibonacci_on_the_fly(values, number - 1)
        + fibonacci_on_the_fly(values, number - 2);
      values.push(value);
      value
    }
  }
}

/// Gets a list of Fibonacci numbers from 0 up to 92
///
/// # Returns
///
/// A list containing the Fibonacci values from F(0) up to F(92).
fn fibonacci_prefill_vector() -> Vec<i64> {
  // Base case
  let mut values: Vec<i64> = vec![0, 1];
  // To calculate the current Fibonacci number sum the last two Fibonacci numbers stored in the
  // list
  for n in 2..=92 {
    values.push(values[n - 1] + values[n - 2]);
  }
  values
}

/// Calculates the Fibonacci number of the given number.
///
/// # Arguments
///
/// * `number`: Number to calculate its Fibonacci number.
///
/// # Returns
///
/// The Fibonacci number calculated for the given number.
///
/// # Remarks
///
/// panics if `number` is negative.
fn fibonacci_bottom_up(number: i64) -> i64 {
  // Don't allow negative numbers.
  if number < 0 {
    panic!(
      "Fibonacci is not allowed for negative numbers such as {}",
      number
    )
  }

  // Base case
  if number <= 1 {
    return number;
  }

  let mut fib_i_minus_2 = 0i64;
  let mut fib_i_minus_1 = 1i64;
  let mut fib_i = fib_i_minus_1 + fib_i_minus_2;
  for _i in 1i64..number {
    // Calculate this value of fib_i.
    fib_i = fib_i_minus_1 + fib_i_minus_2;

    // Set fib_i_minus_2 and fib_i_minus_1 for the next value.
    fib_i_minus_2 = fib_i_minus_1;
    fib_i_minus_1 = fib_i;
  }
  fib_i
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
fn get_i64(prompt: &str) -> i64 {
  print!("{prompt}");
  io::stdout().flush().unwrap();

  let mut str_value = String::new();
  io::stdin()
    .read_line(&mut str_value)
    .expect("Error reading input");

  str_value
    .trim()
    .parse::<i64>()
    .expect("Error parsing integer")
}

fn main() {
  // Initialize the prefilled vector.
  let prefilled_values: Vec<i64> = fibonacci_prefill_vector();

  // Create a vector for fill-on-the-fly.
  let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

  loop {
    // Prompt the user for n.
    println!("Enter a number N to calculate the Fibonacci value. Enter -1 to exit.\n");
    let n = get_i64("N: ");

    if n < 0 {
      // If n < 0, break out of the loop.
      println!("End. Bye.");
      break;
    } else if n > 92 {
      // If n is too big, say so.
      println!("Sorry, values larger than Fibonacci(92) do not fit in an i64.");
      println!();
    } else {
      // Calculate the Fibonacci number.
      println!("Prefilled:  {}", prefilled_values[n as usize]);
      println!(
        "On the fly: {}",
        fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
      );
      println!("Bottom up:  {}", fibonacci_bottom_up(n));
      println!();
    }
  }
}

#[cfg(test)]
mod tests;
