//! [LeetCode 1550. Three Consecutive Odds](https://leetcode.com/problems/three-consecutive-odds/description/)
//! Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.

use std::cmp;

pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    let mut odd_cnt = 0;
    let mut max_odd = 0;
    for num in arr {
        match num % 2 {
            1 => {
                odd_cnt += 1;
                max_odd = cmp::max(odd_cnt, max_odd);
            }
            0 => odd_cnt = 0,
            _ => unreachable!(),
        }
    }
    max_odd >= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![2, 6, 4, 1];
        assert_eq!(three_consecutive_odds(arr), false);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        assert_eq!(three_consecutive_odds(arr), true);
    }

    #[test]
    fn test_3() {
        let arr = vec![1, 3, 5, 7, 23, 12];
        assert_eq!(three_consecutive_odds(arr), true);
    }
}
