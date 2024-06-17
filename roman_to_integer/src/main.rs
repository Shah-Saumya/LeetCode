// For more details visit 'https://leetcode.com/problems/roman-to-integer/description/'

fn main() {
    println!("Hello, world!");
    let rstr = "ivxlcdm".to_string();
    let solution = Solution::roman_to_int(rstr.clone());
    println!("{}: {}", rstr, solution);
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() == 0 {
            panic!("Roman Number cann't be empty")
        }
        let charset = s.to_uppercase().chars().collect::<Vec<char>>();
        println!("{charset:?}");

        let mut res: usize = 0;

        let _ = charset.into_iter().map(|ch| match ch {
            'I' => res += 1,
            'V' => res += 5,
            'X' => res += 10,
            'L' => res += 50,
            'C' => res += 100,
            'D' => res += 500,
            'M' => res += 1000,
            _ => panic!("Invalid character in string: {}", s),
        }).collect::<Vec<_>>();
        res as i32
    }
}
