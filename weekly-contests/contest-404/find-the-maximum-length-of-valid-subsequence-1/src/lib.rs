//! You are given an integer array nums.
//! A `subsequence` sub of nums with length x is called valid if it satisfies:
//! - `(sub[0] + sub[1]) % 2 == (sub[1] + sub[2]) % 2 == ... == (sub[x - 2] +
//!   sub[x - 1]) % 2.`
//!
//! Return the length of the longest valid subsequence of nums.
//! A `subsequence` is an array that can be derived from another array by
//! deleting some or no elements without changing the order of the remaining
//! elements.

/// # Maximum Length of the Subsequence
///
/// ## Def
///
/// Subsequence: PDEP -> item deposit to a new array.
///
/// ## Attempt 1: DP
///
/// When deriving a subsequence, for each num in the array, I can choose to pick
/// it or not. For all vectors of size `n`, there're `2^n` subsequences.
///
/// If we have solved the max valid subsequence `&nums[1..n]`, we can choose to
/// use `nums[n]` or not.
///
/// If we pick `nums[n]`, we have guarantee that the new subsequence is still
/// valid.
///
/// There are two problems:
///
/// - We don't know the exact subsequence derived from `&nums[1..n]`, we only
///   know the length of that sequence.
/// - There might be multiple subsequence with the same max length in
///   `&nums[1..n]`
///
/// ## Attempt 2
///
/// - All sum of the consecutive numbers in the subsequence will have the same
///   residual mod 2
/// - The residual can be 0 or 1
///   - If the residual is 0, the two numbers must be both even or both odd.
///   - If the residual is 1, the two numbers must be different in terms of mod
///     2.
///
/// Three scenarios:
///
/// - All even
/// - All odd
/// - Alternating even and odd
pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mods: Vec<i32> = nums.iter().map(|x| x % 2).collect();

    // Even subsequence.
    let mut num_even = 0;
    // Odd subsequence.
    let mut num_odd = 0;
    // Alternating subsequence.
    let mut num_shift = 1;
    let mut last_res = mods[0];
    for res in mods {
        match res {
            0 => num_even += 1,
            1 => num_odd += 1,
            _ => unreachable!(),
        }
        if res != last_res {
            num_shift += 1;
            last_res = res;
        }
    }

    let res = vec![num_even, num_odd, num_shift];
    *res.iter().max().unwrap()
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

    #[test]
    fn test_4() {
        let nums = vec![2, 4];
        assert_eq!(maximum_length(nums), 2);
    }
}
