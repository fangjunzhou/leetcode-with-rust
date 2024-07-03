pub fn min_difference(nums: Vec<i32>) -> i32 {
    /// Find the minimum difference of a sorted array within certain moves.
    ///
    /// * `sorted_nums`: a sorted array of numbers.
    /// * `mv`: the number of move left.
    fn solve(sorted_nums: &[i32], mv: i32) -> i32 {
        use std::cmp;

        // Base case.
        if sorted_nums.len() == 1 {
            return 0;
        }
        if mv == 0 {
            return sorted_nums.last().unwrap() - sorted_nums.first().unwrap();
        }
        // Move front or end.
        let mv_front = solve(&sorted_nums[1..], mv - 1);
        let mv_end = solve(&sorted_nums[..sorted_nums.len() - 1], mv - 1);

        return cmp::min(mv_front, mv_end);
    }

    let mut nums = nums.clone();
    nums.sort();
    return solve(&nums, 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_difference() {
        let nums = vec![5, 3, 2, 4];
        assert_eq!(min_difference(nums), 0);
        let nums = vec![1, 5, 0, 10, 14];
        assert_eq!(min_difference(nums), 1);
        let nums = vec![3, 100, 20];
        assert_eq!(min_difference(nums), 0);
        let nums = vec![2];
        assert_eq!(min_difference(nums), 0);
        let nums = vec![2, 3];
        assert_eq!(min_difference(nums), 0);
        let nums = vec![0, 5, 10, 15, 20, 25, 30];
        assert_eq!(min_difference(nums), 15);
    }
}
