use std::collections::HashSet;

fn is_happy(n: i32) -> bool {
    let mut set = HashSet::new();

    fn square_sum(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            let digit = num % 10;
            sum += digit * digit;
            num /= 10;
        }
        sum
    }

    let mut n = n;
    while n != 1 && !set.contains(&n) {
        set.insert(n);
        n = square_sum(n);
    }

    n == 1
}

fn main() {
    // Example usage:
    println!("{}", is_happy(19)); // Output: true
    println!("{}", is_happy(2)); // Output: false
}
