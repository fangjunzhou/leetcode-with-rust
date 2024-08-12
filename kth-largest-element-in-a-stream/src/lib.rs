//! [LeetCode 703](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

use std::usize;

struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_by(|a, b| b.partial_cmp(a).unwrap());
        Self { k, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        // Empty stream.
        if self.nums.len() == 0 {
            self.nums.push(val);
            return val;
        }
        // Find the index to insert value.
        let mut l = 0;
        let mut r = self.nums.len() - 1;
        while l != r {
            let mid = (l + r) / 2;
            if self.nums[mid] > val {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if self.nums[l] > val {
            self.nums.insert(l + 1, val);
        } else {
            self.nums.insert(l, val);
        }
        return self.nums[self.k as usize - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut kth_argest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_argest.add(3), 4);
        assert_eq!(kth_argest.add(5), 5);
        assert_eq!(kth_argest.add(10), 5);
        assert_eq!(kth_argest.add(9), 8);
        assert_eq!(kth_argest.add(4), 8);
    }
}
