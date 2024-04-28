//! This program calculates the nth Fibonacci number using both unoptimized and optimized approaches.
//!
//! The `fib_unoptimized` function calculates the nth Fibonacci number using a simple recursive approach,
//! resulting in exponential time complexity.
//!
//! The `fib_optimized` function calculates the nth Fibonacci number using dynamic programming with memoization,
//! resulting in improved time complexity by avoiding redundant calculations.
//!
//! The main function demonstrates the difference in execution time between the two approaches for calculating
//! the Fibonacci number at the 40th position.

use std::collections::HashMap;
use std::time::{ Instant };

fn main() {
    let mut cache: HashMap<u8, u128> = HashMap::new();
    let term = 40;

    let mut start = Instant::now();
    let result = fib_unoptimized(term);
    let mut duration = start.elapsed();
    println!("The nth value is: {}, time taken: {:?}", result, duration);

    start = Instant::now();
    let result = fib_optimized(term, &mut cache);
    duration = start.elapsed();
    println!("The nth value is: {}, time taken: {:?}", result, duration);
}

fn fib_unoptimized(n: u8) -> u128 {
    if n == 1 || n == 2 { 1 } else { fib_unoptimized(n - 1) + fib_unoptimized(n - 2) }
}

fn fib_optimized(n: u8, cache: &mut HashMap<u8, u128>) -> u128 {
    if n == 1 || n == 2 {
        1
    } else {
        if let Some(&value) = cache.get(&n) {
            value
        } else {
            let value1 = fib_optimized(n - 1, cache);
            let value2 = fib_optimized(n - 2, cache);
            let value = value1 + value2;
            cache.insert(n, value);
            value
        }
    }
}
