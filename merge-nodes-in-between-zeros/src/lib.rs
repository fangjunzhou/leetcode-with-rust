//! [LeetCode 2181](https://leetcode.com/problems/merge-nodes-in-between-zeros/)

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Construct a new linked list from an i32 slice.
    ///
    /// * `arr`: the array slice representation of the linked list.
    fn from(arr: &[i32]) -> Option<Box<Self>> {
        // Base case.
        if arr.len() == 0 {
            return None;
        }
        // Construct the node.
        let mut node = Box::new(ListNode::new(arr[0]));
        node.next = Self::from(&arr[1..]);
        return Some(node);
    }
}

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Setup current node.
    let mut curr = head.unwrap().next;
    // Base case.
    if &curr == &None {
        return None;
    }
    // Result node.
    let mut res = Box::new(ListNode::new(0));
    loop {
        match curr {
            Some(node) => {
                if node.val == 0 {
                    curr = Some(node);
                    break;
                }
                res.val += node.val;
                curr = node.next;
            }
            None => unreachable!(),
        }
    }
    res.next = merge_nodes(curr);
    // Keep reading until get another 0 node.
    return Some(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listnode_from() {
        let list = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        assert_eq!(ListNode::from(&vec![0, 1, 2, 3]), list);
    }

    #[test]
    fn test_merge_nodes() {
        let list = ListNode::from(&vec![0, 3, 1, 0, 4, 5, 2, 0]);
        assert_eq!(merge_nodes(list), ListNode::from(&vec![4, 11]));
        let list = ListNode::from(&vec![0, 1, 0, 3, 0, 2, 2, 0]);
        assert_eq!(merge_nodes(list), ListNode::from(&vec![1, 3, 4]));
        let list = ListNode::from(&vec![0, 1, 0]);
        assert_eq!(merge_nodes(list), ListNode::from(&vec![1]));
    }
}
