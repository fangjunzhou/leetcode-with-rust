//! [LeetCode 2678](https://leetcode.com/problems/number-of-senior-citizens/)

pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut res = 0;
    for detail in details {
        let age_str = &detail[11..=12];
        let age: u32 = age_str.parse().unwrap();
        if age > 60 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let details = vec![
            "7868190130M7522".to_owned(),
            "5303914400F9211".to_owned(),
            "9273338290F4010".to_owned(),
        ];
        assert_eq!(count_seniors(details), 2);
    }

    #[test]
    fn test_2() {
        let details =
            vec!["1313579440F2036".to_owned(), "2921522980M5644".to_owned()];
        assert_eq!(count_seniors(details), 0);
    }
}
