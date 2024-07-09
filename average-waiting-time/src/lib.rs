//! [LeetCode 1701](https://leetcode.com/problems/average-waiting-time/)

use std::cmp;

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut finish = 0;
    let mut wait: Vec<i32> = Vec::new();
    for customer in &customers {
        let arrival = customer[0];
        let time = customer[1];
        finish = cmp::max(finish + time, arrival + time);
        wait.push(finish - arrival);
    }

    let s: f64 = wait.iter().map(|x| f64::from(*x)).sum();
    return s / f64::from(customers.len() as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
        assert_eq!(average_waiting_time(customers), 5.0);
    }

    #[test]
    fn test_2() {
        let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
        assert_eq!(average_waiting_time(customers), 3.25);
    }
}
