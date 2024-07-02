use std::collections::HashSet;
use std::collections::VecDeque;

/// Construct a neighbor lookup table for a tree.
///
/// * `edges`: all the bidirectional edges.
fn get_neighbors(edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // The result neighbor graph.
    let mut res: Vec<Vec<i32>> = vec![vec![]; edges.len() + 1];
    for edge in edges {
        let from = edge[0];
        let to = edge[1];
        res[from as usize].push(to);
        res[to as usize].push(from);
    }
    res
}

/// Find the furthest node current node can reach and the path to that node.
///
/// * `neighbors`: the neighbor lookup table of the graph.
/// * `curr`: current node.
fn bfs(neighbors: &Vec<Vec<i32>>, curr: i32) -> (i32, Vec<i32>) {
    // BFS
    let mut visited: HashSet<i32> = HashSet::new();
    let mut queue = VecDeque::from([(curr, vec![curr])]);
    while queue.len() != 0 {
        let (curr, path) = queue.pop_front().unwrap();
        visited.insert(curr);
        // Push all the neighbors.
        for neighbor in &neighbors[curr as usize] {
            if visited.contains(neighbor) {
                continue;
            }
            let mut neighbor_path = path.clone();
            neighbor_path.push(*neighbor);
            queue.push_back((*neighbor, neighbor_path));
        }
        // Check if current node is the last node in the search.
        if queue.len() == 0 {
            return (curr, path);
        }
    }
    (curr, vec![curr])
}

/// Find the diameter and the center of the tree. Return the (diameter, center)
///
/// * `neighbors`: the neighbor lookup table of the tree.
fn diameter_center(neighbors: &Vec<Vec<i32>>) -> (i32, i32) {
    // Reach the boundary
    let (boundary, _) = bfs(neighbors, 0);
    // Do another pass.
    let (_, path) = bfs(neighbors, boundary);
    // Find the center.
    let center = path[path.len() / 2];
    ((path.len() - 1) as i32, center)
}

pub fn minimum_diameter_after_merge(
    edges1: Vec<Vec<i32>>,
    edges2: Vec<Vec<i32>>,
) -> i32 {
    // 1. Build neighbors lookup table.
    let neighbor1 = get_neighbors(&edges1);
    let neighbor2 = get_neighbors(&edges2);
    // 2. Find the diameter and center of two graphs.
    let (diameter1, center1) = diameter_center(&neighbor1);
    let (diameter2, center2) = diameter_center(&neighbor2);
    // 3. Find merged dimeter.
    // 3.1. Find the depth of two tree.
    let (_, path1) = bfs(&neighbor1, center1);
    let depth1 = (path1.len() - 1) as i32;
    let (_, path2) = bfs(&neighbor2, center2);
    let depth2 = (path2.len() - 1) as i32;
    // 3.2 Compute the merged diameter.
    let diameter3 = depth1 + depth2 + 1;

    *vec![diameter1, diameter2, diameter3].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbor() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let neighbor_ref = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        assert_eq!(get_neighbors(&edges), neighbor_ref);
        let edges = vec![vec![0, 1]];
        let neighbor_ref = vec![vec![1], vec![0]];
        assert_eq!(get_neighbors(&edges), neighbor_ref);
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let neighbor_ref = vec![
            vec![1, 2, 3],
            vec![0],
            vec![0, 4, 5, 7],
            vec![0, 6],
            vec![2],
            vec![2],
            vec![3],
            vec![2],
        ];
        assert_eq!(get_neighbors(&edges), neighbor_ref);
    }

    #[test]
    fn test_bfs() {
        let neighbor = vec![
            vec![1, 2, 3],
            vec![0],
            vec![0, 4, 5, 7],
            vec![0, 6],
            vec![2],
            vec![2],
            vec![3],
            vec![2],
        ];
        let (node, path) = bfs(&neighbor, 0);
        assert!(vec![4, 5, 6, 7].contains(&node));
        assert_eq!(path.len(), 3);
        let (node, path) = bfs(&neighbor, 2);
        assert!(vec![6].contains(&node));
        assert_eq!(path.len(), 4);
        let (node, path) = bfs(&neighbor, 6);
        assert!(vec![4, 5, 7].contains(&node));
        assert_eq!(path.len(), 5);
        let (node, path) = bfs(&neighbor, 7);
        assert_eq!(node, 6);
        assert_eq!(path, vec![7, 2, 0, 3, 6]);
    }

    #[test]
    fn test_diameter_center() {
        let neighbor = vec![
            vec![1, 2, 3],
            vec![0],
            vec![0, 4, 5, 7],
            vec![0, 6],
            vec![2],
            vec![2],
            vec![3],
            vec![2],
        ];
        let (diameter, center) = diameter_center(&neighbor);
        assert_eq!(diameter, 4);
        assert_eq!(center, 0);
        let neighbor = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
        let (diameter, center) = diameter_center(&neighbor);
        assert_eq!(diameter, 2);
        assert_eq!(center, 0);
        let neighbor = vec![vec![1], vec![0]];
        let (diameter, center) = diameter_center(&neighbor);
        assert_eq!(diameter, 1);
        assert_eq!(center, 0);
    }

    #[test]
    fn test_minimum_diameter_after_merge() {
        let edges1 = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let edges2 = vec![vec![0, 1]];
        assert_eq!(minimum_diameter_after_merge(edges1, edges2), 3);
        let edges1 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        let edges2 = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 6],
            vec![2, 7],
        ];
        assert_eq!(minimum_diameter_after_merge(edges1, edges2), 5);
    }
}
