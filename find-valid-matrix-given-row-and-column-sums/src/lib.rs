//! [LeetCode 1605](https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/)

pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    // Sort rows and columns.
    let mut row_sum: Vec<(usize, i32)> =
        row_sum.into_iter().enumerate().collect();
    let mut col_sum: Vec<(usize, i32)> =
        col_sum.into_iter().enumerate().collect();
    row_sum.sort_by_key(|x| x.1);
    col_sum.sort_by_key(|x| x.1);
    // Solve the matrix.
    let mut mat = vec![vec![0; col_sum.len()]; row_sum.len()];
    let mut i = 0;
    let mut j = 0;
    while i < row_sum.len() && j < col_sum.len() {
        let (row_idx, mut row_num) = row_sum[i];
        let (col_idx, mut col_num) = col_sum[j];
        let num = match row_num.cmp(&col_num) {
            std::cmp::Ordering::Greater => col_num,
            _ => row_num,
        };
        mat[row_idx][col_idx] = num;
        row_num -= num;
        col_num -= num;
        row_sum[i] = (row_idx, row_num);
        col_sum[j] = (col_idx, col_num);
        if row_num == 0 {
            i += 1;
        }
        if col_num == 0 {
            j += 1;
        }
    }
    return mat;
}

#[cfg(test)]
mod tests {
    use super::*;
}
