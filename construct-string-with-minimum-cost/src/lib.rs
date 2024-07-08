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
}
