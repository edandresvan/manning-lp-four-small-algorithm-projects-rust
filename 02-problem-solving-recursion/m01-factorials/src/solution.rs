// Factorial

fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
        //println!("{}! = {}", n, iterative_factorial(n));
    }
}

// Calculate the factorial recursively.
fn factorial(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    return n * factorial(n - 1);
}

// Calculate the factorial iteratively.
fn iterative_factorial(n: i64) -> i64 {
    let mut result = 1i64;
    for i in 2i64..=n {
        result *= i;
    }

    return result;
}
