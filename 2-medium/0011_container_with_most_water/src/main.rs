// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
// Return the maximum amount of water a container can store.
// Notice that you may not slant the container.

fn main() {
    let arr: Vec<i32> = vec![1, 2, 1];
    println!(
        "Result of '{:?}' vector: {}",
        arr.clone(),
        Solution::max_area(arr)
    );
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_vol = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left != right {
            let i: i32 = height[left];
            let j = height[right];
            let distance = right - left;
            let shorter = i.min(j);

            let volume = (distance) as i32 * shorter;

            max_vol = volume.max(max_vol);

            if shorter == j { right -= 1 } else { left += 1 }
        }

        max_vol
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_01() {
        let vec = vec![1, 2, 1];
        let expected = 2;

        assert_eq!(Solution::max_area(vec), expected);
    }

    #[test]
    fn test_case_02() {
        let vec = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        assert_eq!(Solution::max_area(vec), expected);
    }

    #[test]
    fn test_case_03() {
        let vec = vec![1, 1];
        let expected = 1;

        assert_eq!(Solution::max_area(vec), expected);
    }
}
