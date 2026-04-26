// # 0004 Median of Two Sorted Arrays
// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)) and
// you may assume nums1 and nums2 cannot both be empty.

fn main() {
    // Example 1: nums1 = [1,3], nums2 = [2]
    let nums1 = vec![1, 3, 4];
    let nums2 = vec![2, 5];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Median: {}", result);

    // Example 2: nums1 = [1,2], nums2 = [3,4]
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Median: {}", result);
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1.clone();
        nums.append(&mut nums2.clone());
        nums.sort();
        let n_len = nums.len();
        if n_len % 2 == 0 {
            ((nums[n_len / 2] + nums[(n_len / 2) - 1]) as f64) / 2.0
        } else {
            nums[n_len / 2] as f64
        }
    }
}
