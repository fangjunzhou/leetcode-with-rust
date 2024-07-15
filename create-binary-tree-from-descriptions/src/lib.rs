//! [LeetCode 2196](https://leetcode.com/problems/create-binary-tree-from-descriptions/)

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn create_binary_tree(
    descriptions: Vec<Vec<i32>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let mut child_map: HashMap<i32, (Option<i32>, Option<i32>)> =
        HashMap::new();
    let mut parent_map: HashMap<i32, i32> = HashMap::new();
    for description in &descriptions {
        let parent = description[0];
        let child = description[1];
        let is_left = description[2];
        // Add parent.
        parent_map.insert(child, parent);
        // Add child.
        child_map
            .entry(parent)
            .and_modify(|children| match is_left {
                1 => children.0 = Some(child),
                0 => children.1 = Some(child),
                _ => unreachable!(),
            })
            .or_insert(match is_left {
                1 => (Some(child), None),
                0 => (None, Some(child)),
                _ => unreachable!(),
            });
    }

    // Construct tree.
    fn construct_tree(
        child_map: &HashMap<i32, (Option<i32>, Option<i32>)>,
        root: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let children = child_map.get(&root);
        let (left, right) = match children {
            Some(x) => x,
            None => &(None, None),
        };
        let res = Rc::new(RefCell::new(TreeNode::new(root)));
        res.borrow_mut().left = match left {
            Some(node) => construct_tree(&child_map, *node),
            None => None,
        };
        res.borrow_mut().right = match right {
            Some(node) => construct_tree(&child_map, *node),
            None => None,
        };
        return Some(res);
    }

    // Find root.
    let mut root = descriptions[0][0];
    while let Some(parent) = parent_map.get(&root) {
        root = *parent;
    }
    return construct_tree(&child_map, root);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        dbg!(create_binary_tree(descriptions));
    }

    #[test]
    fn test_2() {
        let mut descriptions = Vec::new();
        for i in 0..10000 {
            descriptions.push(vec![i, i + 1, 1]);
        }
        dbg!(create_binary_tree(descriptions));
    }
}
