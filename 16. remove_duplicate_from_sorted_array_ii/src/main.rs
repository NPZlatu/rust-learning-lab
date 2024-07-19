fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut count = 1;
    let mut j = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        if count <= 2 {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let len = remove_duplicates(&mut nums);
    println!("Modified array: {:?}", &nums[..len]);
    println!("Length of modified array: {}", len);
}
