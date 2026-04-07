// # 0242 Given two strings s and t, return true if t is an anagram of s, and false otherwise.

fn main() {
    let s = String::from("hello");
    let t = String::from("leloh");
    println!("{}", Solution::is_anagram(s, t));
}

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_hashmap: HashMap<char, i32> = HashMap::new();
        let mut t_hashmap: HashMap<char, i32> = HashMap::new();

        Solution::numerate(&s, &mut s_hashmap);
        Solution::numerate(&t, &mut t_hashmap);

        s_hashmap == t_hashmap
    }

    fn numerate(st: &String, hm: &mut HashMap<char, i32>) {
        let s1 = st.chars().enumerate();
        s1.for_each(|ch| {
            let key = ch.1;
            let value = hm.get(&key);

            match value {
                Some(val) => {
                    hm.insert(key, *val + 1);
                }
                None => {
                    hm.insert(key, 1);
                }
            }
        });
    }
}
