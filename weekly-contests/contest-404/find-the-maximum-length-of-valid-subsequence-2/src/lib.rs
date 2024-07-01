//! [LeetCode 3202. Find the Maximum Length of Valid Subsequence 2](https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/)
//!
//! You are given an integer array `nums` and a positive integer `k`.
//! A `subsequence` `sub` of `nums` with length `x` is called valid if it
//! satisfies:
//! - `(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] +
//!   sub[x - 1]) % k.`
//! Return the length of the longest valid subsequence of `nums`.

/// # Find the Maximum Length of Valid Subsequence
pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        assert_eq!(maximum_length(nums, k), 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 4, 2, 3, 1, 4];
        let k = 3;
        assert_eq!(maximum_length(nums, k), 4);
    }
}
