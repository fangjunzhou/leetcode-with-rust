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
//! ```no_run
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

use std::cmp;

#[derive(Debug)]
/// A trie node (or root)
///
/// * `cost`: the cost of the word.
/// * `eow`: if current node is end of word.
/// * `children`: all the children of the node.
struct Trie {
    cost: i32,
    eow: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    /// Construct a new root node for trie.
    fn new() -> Box<Trie> {
        const CHILD: Option<Box<Trie>> = None;
        Box::new(Trie {
            cost: 0,
            eow: false,
            children: [CHILD; 26],
        })
    }

    /// Update the cost of a word.
    ///
    /// * `str`: the string slice of the word.
    /// * `cost`: the cost of the word.
    fn insert(&mut self, s: &str, cost: i32) {
        // Base case: update the cost.
        if s.len() == 0 {
            match self.eow {
                true => {
                    self.cost = cmp::min(self.cost, cost);
                }
                false => {
                    self.eow = true;
                    self.cost = cost;
                }
            }
            return;
        }
        // Update the child.
        let idx = s.as_bytes()[0] - 'a' as u8;
        assert!(idx < 26);
        if let None = &self.children[idx as usize] {
            self.children[idx as usize] = Some(Self::new());
        }
        self.children[idx as usize]
            .as_mut()
            .unwrap()
            .insert(&s[1..], cost);
    }

    /// Find the cost of a string.
    ///
    /// * `s`: the string to find.
    fn search(&self, s: &str) -> Option<i32> {
        // Base case.
        if s.len() == 0 {
            return match self.eow {
                true => Some(self.cost),
                false => None,
            };
        }

        // Check child.
        let idx = s.as_bytes()[0] - 'a' as u8;
        assert!(idx < 26);
        return match &self.children[idx as usize] {
            Some(n) => n.search(&s[1..]),
            None => None,
        };
    }
}

pub fn minimum_cost(
    target: String,
    words: Vec<String>,
    costs: Vec<i32>,
) -> i32 {
    // Construct the trie.
    let mut trie = Trie::new();
    for i in 0..words.len() {
        trie.insert(&words[i], costs[i]);
    }
    // DP.
    let n = target.len();
    let mut dp = vec![0; n];
    for i in (0..n).rev() {
        let mut min_cost: Option<i32> = None;
        // Entire string match.
        if let Some(cost) = trie.search(&target[i..]) {
            min_cost = Some(cost);
        }
        // Match prefix in trie.
        for j in i + 1..n {
            if let Some(cost) = trie.search(&target[i..j]) {
                if dp[j] != -1 {
                    min_cost = match min_cost {
                        None => Some(cost + dp[j]),
                        Some(c) => Some(cmp::min(c, cost + dp[j])),
                    }
                }
            }
        }
        // Result
        dp[i] = match min_cost {
            Some(c) => c,
            None => -1,
        }
    }

    return dp[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("and", 69);
        trie.insert("ant", 42);
        trie.insert("dad", 37);
        trie.insert("do", 73);
        trie.insert("and", 42);
        assert_eq!(trie.search("an"), None);
        assert_eq!(trie.search("and"), Some(42));
        assert_eq!(trie.search("ant"), Some(42));
    }

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

    #[test]
    fn test_2() {
        let target = String::from("aaaa");
        let words =
            vec![String::from("z"), String::from("zz"), String::from("zzz")];
        let costs = vec![1, 10, 100];
        assert_eq!(minimum_cost(target, words, costs), -1);
    }

    #[test]
    fn test_3() {
        let target = String::from("zpeapbke");
        let words = vec![String::from("zpeapbke"), String::from("z")];
        let costs = vec![8, 1];
        assert_eq!(minimum_cost(target, words, costs), 8);
    }
}
