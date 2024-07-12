//! [LeetCode 1598](https://leetcode.com/problems/crawler-log-folder/)

use std::cmp;

pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut depth = 0;
    for log in logs {
        match log.as_str() {
            "./" => {}
            "../" => depth = cmp::max(0, depth - 1),
            _ => depth += 1,
        };
    }
    return depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let logs: Vec<String> = vec!["d1/", "d2/", "../", "d21/", "./"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(min_operations(logs), 2);
    }

    #[test]
    fn test_2() {
        let logs: Vec<String> = vec!["d1/", "d2/", "./", "d3/", "../", "d31/"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(min_operations(logs), 3);
    }

    #[test]
    fn test_3() {
        let logs: Vec<String> = vec!["d1/", "../", "../", "../"]
            .iter()
            .map(|s| String::from(*s))
            .collect();
        assert_eq!(min_operations(logs), 0);
    }
}
