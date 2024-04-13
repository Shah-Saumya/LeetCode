// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"

fn main() {
    let string = "JUSTARANDOMSTRING".to_string();
    println!("Result: {}", Solution::convert(string, 4));
}

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s == "".to_string() || num_rows == 1 {
            return s;
        }

        let mut retvec: Vec<Vec<char>> = Vec::new();

        for _ in 0..num_rows {
            retvec.push(Vec::new());
        }

        let strvec = s.chars().collect::<Vec<_>>();

        if strvec.len() == num_rows as usize {
            return s;
        }

        let mut op = "add";

        let mut vecindex = 0;

        let mut strindex = 0;

        loop {
            if strindex == strvec.len() {
                break;
            }
            match op {
                "add" => {
                    if (vecindex as i32) == num_rows {
                        op = "sub";
                        vecindex -= 2;
                        continue;
                    }
                    retvec[vecindex as usize].push(strvec[strindex]);

                    strindex += 1;

                    vecindex += 1;
                }
                "sub" => {
                    if (vecindex as i32) < 0 {
                        op = "add";
                        vecindex += 2;
                        continue;
                    }
                    retvec[vecindex as usize].push(strvec[strindex]);

                    strindex += 1;

                    vecindex -= 1;
                }
                _ => unreachable!(),
            }
        }

        let mut retstr = "".to_string();

        for i in retvec {
            for ch in i {
                retstr += &ch.to_string();
            }
        }

        retstr
    }
}
