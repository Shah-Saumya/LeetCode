// # 1 Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

fn main() {
    let vec = vec![2, 5, 5, 11];
    let target = 10;
    let tar_vec = Solution::two_sum(vec, target);
    println!("{:?}", tar_vec);
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res_vec = Vec::new();

        'outer: for p in 0..nums.len() {
            for i in 1..nums.len() {
                if (nums[p] + nums[i]) as i32 == target && p != i {
                    res_vec.push(p as i32);
                    res_vec.push(i as i32);
                    break 'outer;
                }
            }
        }

        res_vec
    }
}
