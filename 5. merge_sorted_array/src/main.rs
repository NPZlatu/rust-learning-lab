// You are given two integer arrays nums1 and nums2,
// sorted in non-decreasing order, and two integers m and n,
// representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function,
// but instead be stored inside the array nums1.
// To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged,
// and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

fn main() {
    let mut nums1: Vec<u32> = vec![0];
    let m = 0;
    let mut nums2: Vec<u32> = vec![1];
    let n = 1;

    merge_sorted_array(&mut nums1, &mut nums2, m, n);

    println!("{:?}", nums1)
}

fn merge_sorted_array(nums1: &mut Vec<u32>, nums2: &mut Vec<u32>, m: usize, n: usize) {
    let mut i = (m as isize) - 1;
    let mut j = (n as isize) - 1;
    let mut k = ((m + n) as isize) - 1;

    while k >= 0 {
        if j < 0 && i >= 0 {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else if j >= 0 && i < 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        } else {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
        }

        k -= 1;
    }
}
