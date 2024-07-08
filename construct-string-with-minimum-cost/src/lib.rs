//! [LeetCode 3213](https://leetcode.com/problems/construct-string-with-minimum-cost/)
//!
//! # Problem Description
//!
//! - I have a `target`, a list of word `words`, and the corresponding cost
//!   `costs`.
//! - I want to construct the `target` using `words` with minimum cost.
//! - Notice:
//!   - Each word can be used multiple times.
//!   - Multiple words with same/different costs can exist in `words` and
//!     `costs`
//! - If the `target` cannot be constructed with `words`, return -1
//!
//! ## Examples
//!
//! ```rust
//! use construct_string_with_minimum_cost::minimum_cost;
//!
//! let target = String::from("abcdef");
//! let words = vec![
//!     String::from("abdef"),  // 100
//!     String::from("abc"),    // 1
//!     String::from("d"),      // 1
//!     String::from("def"),    // 10
//!     String::from("ef"),     // 5
//! ];
//! let costs = vec![100, 1, 1, 10, 5];
//! assert_eq!(minimum_cost(target, words, costs), 7);
//! ```
//!
//! # Attempt: DP
//!
//! For each substring `let s = &target[i..]`, we can try to start the
//! construction if `&s[..j]` is in `words` for some `j`. Let the cost of that
//! word to be `c: i32`, the minimum cost of constructing that **substring** `s`
//! is `c + dp[i + j]`.
//!
//! Meanwhile, we may have multiple `j` and cost `c` to start the construction
//! of the substring `s`, we should find the minimal cost among all possible
//! choices and use it as `dp[i]`.
//!
//! ## Formal Description
//!
//! - Let the size of `target` equals `n`.
//! - `dp` is an 1D array of size `n`.
//! - Init:
//!   - If `target[n-1]` is **not** in `words` `dp[n-1] = -1`.
//!   - If `target[n-1]` is in `words`, find all the costs of `target[n-1]`,
//!     `cs`. `dp[n-1] = min(cs)`.
//! - Bellman Equation for `dp[i]`:
//!   - For each `j = i+1...n`, try to find the cost of `target[i:j]`.
//!   - If `target[i:j]` is in `words`, add the cost of `target[i:j]` and
//!     `dp[j]` to `cs`.
//!   - If `cs` is empty, `dp[i] = -1`
//!   - If `cs` is **not** empty, `dp[i] = min(cs)`
//!
//! # Problem: Find the Cost of `target[i:j]`
//!
//! Brute forcing all the words to find the cost of `target[i:j]` is too slow
//! even with DP. This is exactly why I got TLE during the contest.
//!
//! The best way of finding a word in a dictionary is by using a Trie.

pub fn minimum_cost(
    target: String,
    words: Vec<String>,
    costs: Vec<i32>,
) -> i32 {
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let target = String::from("abcdef");
        let words = vec![
            String::from("abdef"),
            String::from("abc"),
            String::from("d"),
            String::from("def"),
            String::from("ef"),
        ];
        let costs = vec![100, 1, 1, 10, 5];
        assert_eq!(minimum_cost(target, words, costs), 7);
    }
}
