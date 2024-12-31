// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

fn main() {
    let house = vec![123, 2, 322, 232, 23, 32, 4, 10];
    let result = Solution::rob(house);
    println!("Result: {}", result);
}

struct Solution;

impl Solution {
    fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let result1 = future(nums.clone(), 0);
        let result2 = future(nums.clone(), 1);

        if result1 > result2 {
            return result1;
        }
        result2
    }
}

fn future(vec: Vec<i32>, index: i32) -> i32 {
    let mut prev1 = 0;
        let mut prev2 = 0;

        for i in (index..vec.len() as i32).rev() {
            let tmp = prev1;
            if prev1 < prev2 + vec[i as usize] {
                prev1 = prev2 + vec[i as usize]
            }
            prev2 = tmp;
        }

        prev1
}