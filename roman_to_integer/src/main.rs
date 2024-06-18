// For more details visit 'https://leetcode.com/problems/roman-to-integer/description/'

fn main() {
    println!("Hello, world!");
    let rstr = "ii".to_string();
    let solution = Solution::roman_to_int(rstr.clone());
    println!("{}: {}", rstr.to_uppercase(), solution);
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let charset = s.to_uppercase().chars().collect::<Vec<char>>();
        if s.len() == 0 {
            panic!("Roman Number cann't be empty")
        }
        if s.len() == 1 {
            return Solution::return_to_roman(charset);
        }
        println!("{charset:?}");

        // let mut res: usize = 0;
        let mut check1 = [0, 0, 0];
        let mut check3 = [0, 0, 0, 0];

        let _ = charset
            .clone()
            .into_iter()
            .map(|ch| match ch {
                'I' => check3[0] += 1,
                'V' => check1[0] += 1,
                'X' => check3[1] += 1,
                'L' => check1[1] += 1,
                'C' => check3[2] += 1,
                'D' => check1[2] += 1,
                'M' => check3[3] += 1,
                _ => panic!("Invalid character in string: {}", s),
            })
            .collect::<Vec<_>>();

        if check1.into_iter().any(|x| x > 1) || check3.into_iter().any(|x| x > 3) {
            panic!("Invalid Roman format!")
        }
        Solution::return_to_roman(charset)
    }
    fn return_to_roman(arr: Vec<char>) -> i32 {
        let mut help = Help {
            num: 0,
            before: "".to_string(),
        };
        arr.into_iter().fold(help.clone(), |h, ch| {
            if h.before == "".to_string() {
                println!("{}", ch);
                match ch {
                    'I' => {
                        help.num = 1;
                        help.before = "I".to_string();
                    }
                    'V' => {
                        help.num = 5;
                        help.before = "V".to_string()
                    }
                    'X' => {
                        help.num = 10;
                        help.before = "X".to_string()
                    }
                    'L' => {
                        help.num = 50;
                        help.before = "L".to_string()
                    }
                    'C' => {
                        help.num = 100;
                        help.before = "C".to_string()
                    }
                    'D' => {
                        help.num = 500;
                        help.before = "D".to_string()
                    }
                    'M' => {
                        help.num = 1000;
                        help.before = "M".to_string()
                    }
                    _ => unreachable!(),
                };
                return help.clone();
            }
            return help.clone();
        });
        return help.num;
    }
}

#[derive(Clone)]
struct Help {
    num: i32,
    before: String,
}
