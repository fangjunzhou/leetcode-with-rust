//! [Leetcode 1. Two Sum](https://leetcode.com/problems/two-sum/)

/// Given an array of integers `nums` and an integer `target`, return indices of
/// the two numbers such that they add up to `target`. You may assume that each
/// input would have exactly one solution, and you may not use the same element
/// twice.
///
/// * `nums`: input array.
/// * `target`: target sum to find.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_nums: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    sorted_nums.sort_by_key(|(_, num)| **num);
    let mut i = 0;
    let mut j = nums.len() - 1;
    while sorted_nums[i].1 + sorted_nums[j].1 != target {
        if sorted_nums[i].1 + sorted_nums[j].1 < target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    vec![sorted_nums[i].0 as i32, sorted_nums[j].0 as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![0, 1])
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![1, 2])
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![0, 1])
    }
}
