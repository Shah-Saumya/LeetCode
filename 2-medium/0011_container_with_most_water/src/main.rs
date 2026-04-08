//

fn main() {
    let arr: Vec<i32> = vec![1, 2, 1];
    println!(
        "Result of '{:?}' vector: {}",
        arr.clone(),
        Solution::max_area(arr)
    );
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let CACHE: HashMap<((usize, &i32), (usize, &i32)), usize> = HashMap::new();
        let mut prod: usize = 0;

        height.iter().enumerate().reduce(|a, b| {
            let prod_a = Solution::find_max(a, height.clone());
            let prod_b = Solution::find_max(b, height.clone());
            if prod_a > prod_b {
                prod = prod_a;
                a
            } else {
                prod = prod_b;
                b
            }
        });

        prod as i32
    }

    fn find_max(a: (usize, &i32), height: Vec<i32>) -> usize {
        let mut prod = 0;
        height
            .iter()
            .enumerate()
            .for_each(|s| {
                let calc = Solution::calculate(a, s);
                if prod <= calc {
                    prod = calc
                }
            });
        prod
    }

    fn calculate(
        from: (usize, &i32),
        to: (usize, &i32),
    ) -> usize {        
        let length = if from.0 > to.0 {
            (from.0 + 1) - (to.0 + 1)
        } else {
            (to.0 + 1) - (from.0 + 1)
        };
        let height = if from.1 < to.1 { from.1 } else { to.1 };
        let prod = length * (*height as usize);
        prod
    }
}
