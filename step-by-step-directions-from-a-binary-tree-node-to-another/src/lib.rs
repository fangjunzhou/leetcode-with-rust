//! [LeetCode 2096](https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/)

use std::cell::RefCell;
use std::rc::Rc;

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

pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<String> {
        let root = root.as_ref()?.clone();
        // Base case.
        if root.borrow().val == target {
            return Some(String::from(""));
        }
        if let Some(mut left) = dfs(&root.borrow().left, target) {
            left.push('L');
            return Some(left);
        }
        if let Some(mut right) = dfs(&root.borrow().right, target) {
            right.push('R');
            return Some(right);
        }
        return None;
    }
    let start_path = dfs(&root, start_value).unwrap();
    let mut start_iter = start_path.chars().rev();
    let dest_path = dfs(&root, dest_value).unwrap();
    let mut dest_iter = dest_path.chars().rev();
    loop {
        let start = start_iter.next();
        let dest = dest_iter.next();
        if start == None {
            let mut res = String::from(dest.unwrap());
            res.extend(dest_iter);
            return res;
        }
        if dest == None {
            let mut res = String::from('U');
            while let Some(_) = start_iter.next() {
                res.push('U');
            }
            return res;
        }
        let start = start.unwrap();
        let dest = dest.unwrap();
        if start != dest {
            let mut res = String::from('U');
            while let Some(_) = start_iter.next() {
                res.push('U');
            }
            res.push(dest);
            res.extend(dest_iter);
            return res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
