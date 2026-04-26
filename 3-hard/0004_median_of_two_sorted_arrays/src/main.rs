// # 0004 Median of Two Sorted Arrays
// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)) and
// you may assume nums1 and nums2 cannot both be empty.

fn main() {
    // Example 1: nums1 = [1,3], nums2 = [2]
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
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
        // TODO: Implement your solution here
        // Hint: You may need to merge the arrays or use binary search
        // for optimal O(log(m+n)) complexity
        todo!()
    }
}
