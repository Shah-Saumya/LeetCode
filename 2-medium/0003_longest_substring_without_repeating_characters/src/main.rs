// # 0003 Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest substring without repeating characters.

fn main() {
    let result = Solution::length_of_longest_substring("sdfkjhsdkfhsjfhsdhh".to_string());
    println!("{result}");
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        println!("Start of the input function\n\tString: {s}");
        if s.len() == 1 {
            return 1;
        }
        let rev_s = Solution::reverse_string(s.clone());
        let str_vec = Solution::get_str_vec(s);
        let rev_str_vec = Solution::get_str_vec(rev_s);
        let max = Solution::get_max_len(str_vec);
        let rev_max = Solution::get_max_len(rev_str_vec);
        if max > rev_max { max } else { rev_max }
    }

    fn get_max_len(vec: Vec<String>) -> i32 {
        if vec.len() == 0 {
            return 0;
        }
        if vec.len() == 1 {
            return vec[0].len() as i32;
        }
        let answer_str = vec
            .iter()
            .reduce(|a, b| {
                if a.len() > b.len() {
                    return a;
                } else {
                    return b;
                }
            })
            .unwrap();
        answer_str.len() as i32
    }

    fn reverse_string(s: String) -> String {
        let mut rev = String::new();
        // for i in s.len()..0 {
        //     rev.push(s);
        // }
        rev
    }

    fn get_str_vec(s: String) -> Vec<String> {
        println!("Start of the function get_str_vec\n\t-> String: {s}");
        let mut str_vec: Vec<String> = Vec::new();
        let mut temp_str = String::new();
        println!("The loop is starting...");
        for (i, char) in s.chars().enumerate() {
            println!("\t-> {char}");
            if temp_str.len() == 0 {
                temp_str.push(char);
            } else if temp_str.contains(char) {
                str_vec.push(temp_str.clone());
                temp_str = String::from(char);
            } else if i == s.len() - 1 {
                temp_str.push(char);
                str_vec.push(temp_str.clone());
            } else {
                temp_str.push(char);
            }
            println!("\t--> {temp_str}");
        }
        println!("The loop has ended\n\tString Vector: {:?}", str_vec);
        str_vec
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
}
