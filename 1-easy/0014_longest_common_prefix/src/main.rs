// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

fn main() {
    let strs = vec!["car".to_string(), "cir".to_string()];

    let res = Solution::longest_common_prefix(strs);
    println!("Longest common prefix: '{}'", res);
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.clone().into_iter().fold(strs[0].clone(), |acc, s| {
            let (first, second) = (
                acc.chars().collect::<Vec<_>>(),
                s.chars().collect::<Vec<_>>(),
            );
            let mut ret = "".to_string();
            for (i, v) in first.into_iter().enumerate() {
                if i >= second.len() || v != second[i] {
                    return ret;
                } else if v == second[i] {
                    ret += &v.to_string();
                }
            }
            return ret;
        })
    }
}
