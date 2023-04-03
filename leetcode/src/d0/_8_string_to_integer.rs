struct Solution;
use std::i32;

const RADIX: u32 = 10;


impl Solution {
    fn my_atoi(s: String) -> i32 {
        let mut start = s.trim_start();
        let mut res: i32 = 0;
        let mut positive = true;
        if start.len() > 1 {
            let c = &start[0..1];
            match c {
                "+" => {
                    start = &start[1..];
                }
                "-" => {
                    start = &start[1..];
                    positive = false;
                }
                _ => {
                    if let Some(c) = c.chars().next() {
                        if !('0'..='9').contains(&c) {
                            return 0;
                        }
                    }
                }
            }
        }
        for c in start.chars() {
            if ('0'..='9').contains(&c) {
                let digit = c.to_digit(RADIX).unwrap() as i32;
                if res > i32::MAX / 10 || 
                (res == i32::MAX / 10 && digit > i32::MAX % 10) {
                    // If integer overflowed return 2^31-1, otherwise if underflowed return -2^31. 
                    return Self::overflow(positive);
                }
                //  Append current digit to the result.
                res = 10 * res + digit;
                
                // res = match res.checked_mul(10) {
                //     None => {
                //         return Self::overflow(positive);
                //     }
                //     Some(val) => val,
                // };
                // res = match res.checked_add((c as u8 - b'0') as i32) {
                //     None => {
                //         return Self::overflow(positive);
                //     }
                //     Some(val) => val,
                // };
            } else {
                break;
            }
        }
        if !positive {
            res = match res.checked_mul(-1) {
                None => {
                    return Self::overflow(positive);
                }
                Some(val) => val,
            };
        }
        return res;
    }

    fn overflow(positive: bool) -> i32 {
        return if positive { i32::MAX } else { i32::MIN };
    }
}

#[test]
fn test() {
    let s = "42".to_string();
    let res = 42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "   -42".to_string();
    let res = -42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "4193 with words".to_string();
    let res = 4193;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "words and 987".to_string();
    let res = 0;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "-91283472332".to_string();
    let res = -2_147_483_648;
    assert_eq!(Solution::my_atoi(s), res);
}
