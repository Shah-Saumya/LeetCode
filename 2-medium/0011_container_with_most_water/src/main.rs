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

use std::collections::HashMap;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut cache: HashMap<((usize, &i32), (usize, &i32)), usize> = HashMap::new();
        let mut prod: usize = 0;

        // height.iter().enumerate().reduce(|a, b| {
            // let calc_a = Solution::find_max(a, height.clone(), cache.clone());
            // let prod_a = calc_a.0;
            // cache = calc_a.1;
            // let calc_b = Solution::find_max(b, height.clone(), cache.clone());
            // let prod_b = calc_b.0;
            // cache = calc_b.1;
            // if prod_a > prod_b {
                // prod = prod_a;
                // a
            // } else {
                // prod = prod_b;
                // b
            // }
        // });

        prod as i32
    }

    // fn find_max<'b>(
    //     a: (usize, &'b i32),
    //     height: Vec<i32>,
    //     mut hm_cache: HashMap<((usize, &'b i32), (usize, &'b i32)), usize>,
    // ) -> (usize, HashMap<((usize, &'b i32), (usize, &'b i32)), usize>) {
    //     let mut prod = 0;
    //     height.iter().enumerate().for_each(|s| { // TODO: lifetime error (`height` does not live long enough borrowed value does not live long enough)
    //         let calculated = Solution::calculate(a, s, hm_cache.clone());
    //         let calc = calculated.0;
    //         hm_cache = calculated.1;
    //         if prod <= calc {
    //             prod = calc
    //         }
    //     });
    //     (prod, hm_cache)
    // }

    // fn calculate<'a>(
    //     from: (usize, &'a i32),
    //     to: (usize, &'a i32),
    //     mut hm_cache: HashMap<((usize, &'a i32), (usize, &'a i32)), usize>,
    // ) -> (usize, HashMap<((usize, &'a i32), (usize, &'a i32)), usize>) {
    //     if hm_cache.contains_key(&(from, to)) {
    //         return (*hm_cache.get(&(from, to)).unwrap(), hm_cache);
    //     }
    //     let length = if from.0 > to.0 {
    //         (from.0 + 1) - (to.0 + 1)
    //     } else {
    //         (to.0 + 1) - (from.0 + 1)
    //     };
    //     let height = if from.1 < to.1 { from.1 } else { to.1 };
    //     let prod = length * (*height as usize);
    //     hm_cache.insert((from, to), prod);
    //     (prod, hm_cache)
    // }
}
