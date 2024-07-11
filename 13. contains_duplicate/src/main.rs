use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for &num in nums.iter() {
        if !set.insert(num) {
            return true;
        }
    }
    false
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 1];
    println!("{}", contains_duplicate(nums)); // Output: true

    let nums = vec![1, 2, 3, 4, 5];
    println!("{}", contains_duplicate(nums)); // Output: false
}
