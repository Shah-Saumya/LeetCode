// Given an integer x, return true if x is a palindrome, and false otherwise.

fn main() {
    let num = 1001;

    let c = is_palindrome(num);

    if c {
        println!("{num} is a palindrome number");
    } else {
        println!("{num} is not a palindrome number");
    }
}

fn is_palindrome(num: i32) -> bool {
    let mut tnum = num;

    let mut new_num: i32 = 0;

    while tnum > 0 {
        new_num = (new_num * 10) + (tnum % 10);
        tnum = tnum / 10;
    }

    if num < 0 {
        false
    } else if num == new_num {
        true
    } else {
        false
    }
}
