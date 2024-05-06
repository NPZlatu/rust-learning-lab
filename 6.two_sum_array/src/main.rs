fn main() {
    println!("Hello, world!");
    let nums: Vec<i32> = vec![2, 7, 11, 15];

    let target = 9;
    println!("{:?}", get_pair(&nums, target));
}

fn get_pair(nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            println!("{:?} ", nums[i] + nums[j]);
            if nums[i] + nums[j] == target {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}
