//! [LeetCode 179](https://leetcode.com/problems/largest-number/)

pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums.clone();
    nums.sort_by(|a, b| {
        let ab = a.to_string() + &b.to_string();
        let ba = b.to_string() + &a.to_string();
        let ab = ab.parse::<u64>().unwrap();
        let ba = ba.parse::<u64>().unwrap();
        return ab.cmp(&ba);
    });
    nums.reverse();
    if nums[0] == 0 {
        return "0".to_owned();
    }
    dbg!(&nums);
    let mut res = String::new();
    for num in nums {
        res.push_str(&num.to_string());
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![10, 2];
        assert_eq!(largest_number(nums), "210");
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 30, 34, 341, 344, 345, 5, 9];
        assert_eq!(largest_number(nums), "9534534434341330");
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 30, 33, 34, 331, 333, 334, 5, 9];
        assert_eq!(largest_number(nums), "953433433333333130");
    }

    #[test]
    fn test_4() {
        let nums = vec![34323, 3432];
        assert_eq!(largest_number(nums), "343234323");
    }

    #[test]
    fn test_5() {
        let nums = vec![999999991, 9];
        assert_eq!(largest_number(nums), "9999999991")
    }
}
