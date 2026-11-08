// # 0003 Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest substring without repeating characters.

fn main() {
    let result = Solution::length_of_longest_substring("dfhf".to_string());
    println!("{result}");
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        println!("Start of the input function\n\tString: {s}");
        if s.len() == 1 {
            return 1;
        }
        let mut max_vec = Vec::new();
        'outer: for i in 0..s.len() {
            let mut temp_str = String::new();
            println!("Temp String: {} | Iter: {}", temp_str, i);
            for (j, ch) in s.chars().enumerate() {
                if j < i {
                    continue;
                }
                if temp_str.len() == 0 {
                    temp_str.push(ch);
                } else if temp_str.contains(ch) {
                    max_vec.push(temp_str.clone());
                    temp_str = String::new();
                    if temp_str.len() > s.len() - i {
                        break 'outer;
                    }
                } else {
                    temp_str.push(ch);
                }
                if j == s.len() - 1 {
                    max_vec.push(temp_str.clone());
                }
                println!("\tTemp String: {} | Iter: {}-{}", temp_str, i, j);
            }
        }
        println!("{:?}", max_vec);
        let max = max_vec.iter().reduce(|a, b| {
            if a.len() > b.len() {
                return a;
            } else {
                return b;
            }
        });
        max.unwrap().len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_case_01() {
        let s = "abcabcbb".to_string();
        let expected = 3;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_02() {
        let s = "pwwkew".to_string();
        let expected = 3;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_03() {
        let s = "bbbbb".to_string();
        let expected = 1;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_04() {
        let s = "mq".to_string();
        let expected = 2;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_05() {
        let s = "1R1T7".to_string();
        let expected = 4;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_06() {
        let s = "afedacd".to_string();
        let expected = 5;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_case_07() {
        let s = "afeda*&^%^%#^&%*&^&^$^#%^&^HJGKJFuhsdfisndfiufeiwufhihksjdfhiuehifuhsidufiuei*&GU&T*SD*W&EYYEGD*&#^DGWUD&#YD#G&EY^G#R*&#YGYDGEWYU&#&^@!(*WEU*#(@#*&hiuhiufhsiufhsfiushfuiwehiuhsiufieufhfsiufcd".to_string();
        let expected = 10;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }
}
