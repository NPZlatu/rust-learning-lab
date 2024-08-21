use std::collections::HashMap;

fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&last_index) = map.get(&num) {
            if (i as i32) - (last_index as i32) <= k {
                return true;
            }
        }
        map.insert(num, i);
    }

    false
}

fn main() {
    let nums = vec![1, 2, 3, 1, 2, 3];
    let k = 2;
    let result = contains_nearby_duplicate(nums, k);
    println!("Contains nearby duplicate: {}", result);
}
