//! [LeetCode 3202. Find the Maximum Length of Valid Subsequence 2](https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/)
//!
//! You are given an integer array `nums` and a positive integer `k`.
//! A `subsequence` `sub` of `nums` with length `x` is called valid if it
//! satisfies:
//! - `(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] +
//!   sub[x - 1]) % k.`
//! Return the length of the longest valid subsequence of `nums`.

/// # Find the Maximum Length of Valid Subsequence
///
/// ## Relationship with [LeetCode 3201](https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/)
///
/// ### What's **Valid**
///
/// - In 3201: `(sub[n] + sub[n+1]) % 2 = (sub[n+1] + sub[n+2]) % 2` for all `n`
/// - In 3202: `(sub[n] + sub[n+1]) % k = (sub[n+1] + sub[n+2]) % k` for all `n`
///
/// ### Residual After `mod`
///
/// - In 3201: `(a + b) % 2` in `[0, 1]`
/// - In 3202: `(a + b) % k` in `[0, 1, ..., k-1]`
///
/// ### Cases to Consider
///
/// - In 3201:
///   - When `(a + b) % 2 = 0`
///     - `a % 2 = 0` and `b % 2 = 0` (a and b are both even)
///     - `a % 2 = 1` and `b % 2 = 1` (a and b are both odd)
///   - When `(a + b) % 2 = 1`
///     - `a % 2 = 0` and `b % 2 = 1` (a is even and b is odd)
///     - `a % 2 = 1` and `b % 2 = 0` (a is odd and b is even)
/// - In 3202:
///   - When `(a + b) % 2 = c`
///     - ???
///
/// ## Key Observation from Discrete Math
///
/// ### Observation I
///
/// `(a + b) % k = (a % k + b % k) % k`
///
/// #### Example
///
/// When `k = 5, a = 69, b = 42`, `(a + b) % k = (69 + 42) % 5 = 111 % 5 = 1`.
/// `69 % 5 = 4, 42 % 5 = 2`. `(69 % 5 + 42 % 5) % 5 = (4 + 2) % 5 = 1`
///
/// ### Observation II
///
/// For each `a` in `[0, ..., k-1]`, there's exactly one `b` in `[0, ..., k-1]`
/// such that `(a + b) % k = c` where `c` is in `[0, ..., k-1]`.
///
/// #### Example
///
/// `k = 5, b = 2`, I want to find `a` in `[0, 1, 2, 3, 4]` such that `(a + b) %
/// 5 = 1`. `a` can only be 4.
///
/// ## The New Alternating Residual Algorithm
///
/// To guarantee that we derive a valid subsequence, all the items in that
/// subsequence will have alternating residuals.
///
/// For example, `[69, 42, 34, 57]` is a valid subsequence for `k = 5`. The
/// residuals of this sequence mod 5 is `[4, 2, 4, 2]`.
///
/// However, when we traverse the array and get to a specific residual, for
/// example, 2, the residual may exist in another sequence.
///
/// For example, if we have `nums = [33, 69, 42, 34, 28, 57]`, we can derive a
/// subseuqnece `[69, 42, 34, 57]` where the residual is `[4, 2, 4, 2]`. But we
/// can also derive another subsequence `[33, 42, 28, 57]` where the residual is
/// `[3, 2, 3, 2]`. Notice item 42 and 57 participate in multiple subsequence.
/// Or, we can say **residual 2 participated in multiple alternating residual
/// sequence**
///
/// Let `ress` be the residuals of `nums` mod `k`. When we get to `x = ress[i]`,
/// x can participate in `k` **alternating residual sequence**.
///
/// We need a 2D array to keep track of each possible alternating residual
/// sequence. Each entry in the array should keep track of:
/// - The last residual in the alternating residual sequence. This record is
///   used to guarantee the residual is alternating.
/// - The length of the sequence. You should use this to find the max
///   subsequence.
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
