//! [LeetCode 1110](https://leetcode.com/problems/delete-nodes-and-return-forest/)

use std::cell::RefCell;
use std::rc::Rc;

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

pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::{HashSet, VecDeque};

    /// Process the node.
    ///
    /// * `curr`: current node.
    /// * `to_delete`: to delete hash set.
    /// * `queue`: working queue.
    fn delete(
        curr: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
        queue: &mut VecDeque<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let curr = curr?;
        // If current node is deleted.
        let val = curr.borrow().val;
        if to_delete.contains(&val) {
            if let Some(left) = curr.borrow_mut().left.take() {
                queue.push_back(Some(left));
            }
            if let Some(right) = curr.borrow_mut().right.take() {
                queue.push_back(Some(right));
            }
            return None;
        }

        // If current node is not deleted.
        let left = curr.borrow_mut().left.take();
        curr.borrow_mut().left = delete(left, to_delete, queue);
        let right = curr.borrow_mut().right.take();
        curr.borrow_mut().right = delete(right, to_delete, queue);

        return Some(curr);
    }

    // Init to_delete HashSet.
    let to_delete: HashSet<i32> = to_delete.into_iter().collect();
    // Init the queue and forest.
    let mut queue = VecDeque::from([root]);
    let mut forest = vec![];
    while let Some(curr) = queue.pop_front() {
        if let Some(node) = delete(curr, &to_delete, &mut queue) {
            forest.push(Some(node));
        }
    }

    return forest;
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! node {
        ($val:literal, $left:expr, $right:expr) => {
            Some(Rc::new(RefCell::new(TreeNode {
                val: $val,
                left: $left,
                right: $right,
            })))
        };
    }

    #[test]
    fn test_1() {
        let root = node![
            1,
            node![2, node![4, None, None], node![5, None, None]],
            node![3, node![6, None, None], node![7, None, None]]
        ];
        let to_delete = vec![3, 5];
        let expect = vec![
            node![1, node![2, node![4, None, None], None], None],
            node![6, None, None],
            node![7, None, None],
        ];
        assert_eq!(del_nodes(root, to_delete), expect);
    }

    #[test]
    fn test_2() {
        let root = node![
            1,
            node![2, None, node![3, None, None]],
            node![4, None, None]
        ];
        let to_delete = vec![3];
        let expect = vec![node![1, node![2, None, None], node![4, None, None]]];
        assert_eq!(del_nodes(root, to_delete), expect);
    }
}
