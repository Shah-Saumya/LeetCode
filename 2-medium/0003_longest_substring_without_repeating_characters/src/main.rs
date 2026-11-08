// # 0003 Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest substring without repeating characters.

fn main() {
    let result = Solution::length_of_longest_substring("dfhf".to_string());
    println!("{result}");
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }

        let mut s_vec = Vec::new();
        s_vec.reserve_exact(s.len());
        s.chars().for_each(|a| s_vec.push(a));

        let mut temp_str = String::new();
        let mut max_len = 0;

        let mut start_ptr = 0;
        let mut moving_ptr = 1;
        while start_ptr != s.len() {
            if temp_str.len() == 0 {
                temp_str.push(s_vec[start_ptr]);
                continue;
            }

            if moving_ptr == s.len() - 1 {
                if !temp_str.contains(s_vec[moving_ptr]) {
                    temp_str.push(s_vec[moving_ptr]);
                }
                max_len = max_len.max(temp_str.len());
                start_ptr += 1;
                moving_ptr = start_ptr + 1;
                continue;
            } else if moving_ptr > s.len() - 1 {
                max_len = max_len.max(temp_str.len());
                start_ptr += 1;
                moving_ptr = start_ptr + 1;
                continue;
            }

            if temp_str.contains(s_vec[moving_ptr]) {
                max_len = max_len.max(temp_str.len());
                temp_str = String::new();
                start_ptr += 1;
                moving_ptr = start_ptr + 1;
            } else {
                temp_str.push(s_vec[moving_ptr]);
                moving_ptr += 1;
            }
        }
        max_len as i32
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
