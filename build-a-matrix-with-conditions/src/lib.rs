//! [LeetCode 2392](https://leetcode.com/problems/build-a-matrix-with-conditions/)

/// Get the topological order or a condition array.
///
/// * `k`: the number of nodes in the graph.
/// * `condition`: the dependency of the nodes.
pub fn topological_sort(k: i32, conditions: Vec<Vec<i32>>) -> Option<Vec<i32>> {
    // Construct the graph.
    let mut children: Vec<Vec<usize>> = vec![vec![]; (k + 1) as usize];
    let mut degree: Vec<usize> = vec![0; (k + 1) as usize];
    for condition in conditions {
        let parent = condition[0] as usize;
        let child = condition[1] as usize;
        // Add children.
        children[parent].push(child);
        // Add degree.
        degree[child] += 1;
    }

    return None;
}

pub fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;
}
