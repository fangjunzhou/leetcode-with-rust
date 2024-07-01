//! You are given an integer array nums.
//! A `subsequence` sub of nums with length x is called valid if it satisfies:
//! - `(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] + sub[x - 1]) % 2.`
//!
//! Return the length of the longest valid subsequence of nums.
//! A `subsequence` is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(maximum_length(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 1, 1, 2, 1, 2];
        assert_eq!(maximum_length(nums), 6);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 3];
        assert_eq!(maximum_length(nums), 2);
    }
}
