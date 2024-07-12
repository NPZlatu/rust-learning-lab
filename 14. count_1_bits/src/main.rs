fn count_bits(n: usize) -> Vec<u32> {
    let mut ans = Vec::with_capacity(n + 1);

    for i in 0..=n {
        ans.push(count_ones(i));
    }

    ans
}

fn count_ones(mut num: usize) -> u32 {
    let mut count = 0;
    while num > 0 {
        if (num & 1) == 1 {
            count += 1;
        }
        num >>= 1;
    }
    count
}

fn main() {
    let n = 2;
    let result = count_bits(n);
    println!("{:?}", result); // Output: [0, 1, 1]
}
