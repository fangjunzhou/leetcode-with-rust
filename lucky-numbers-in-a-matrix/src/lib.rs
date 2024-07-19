//! [LeetCode 1380](https://leetcode.com/problems/lucky-numbers-in-a-matrix/)

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = Vec::new();
    'row_loop: for row in &matrix {
        // Find the smallest in a row.
        let min_num =
            row.into_iter().enumerate().min_by_key(|(idx, num)| **num);
        let (min_idx, min_num) = min_num.unwrap();
        // If the number is maximum in the column, add to result.
        for i in 0..matrix.len() {
            if matrix[i][min_idx] > *min_num {
                continue 'row_loop;
            }
        }
        res.push(*min_num);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
}
