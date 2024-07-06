//! [LeetCode 2058](https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/description/)

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(list: &[i32]) -> Option<Box<Self>> {
        // Base case.
        if list.len() == 0 {
            return None;
        }
        let mut node = Box::new(Self::new(list[0]));
        node.next = Self::from(&list[1..]);
        return Some(node);
    }
}

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    use std::cmp;

    let mut head = head;
    // Consecutive node values.
    let mut vals = vec![0; 2];
    // Load first two nodes.
    for i in 0..2 {
        if let Some(node) = head {
            vals[i] = node.val;
            head = node.next;
        } else {
            return vec![-1, -1];
        }
    }

    // First, last, and current critical point.
    let mut first_cr: Option<i32> = None;
    let mut last_cr: Option<i32> = None;
    let mut min_dist: Option<i32> = None;
    // Traverse the entire linked list.
    let mut idx = 1;
    while let Some(node) = head {
        // Get consecutive values.
        let (last, curr) = (vals[0], vals[1]);
        let next = node.val;

        // Critical point.
        if (curr > last && curr > next) || (curr < last && curr < next) {
            dbg!(idx);
            // Init first critical point.
            first_cr = match first_cr {
                None => Some(idx),
                Some(val) => Some(val),
            };
            // Calculate the min distance.
            if let Some(last_idx) = last_cr {
                let curr_dist = idx - last_idx;
                min_dist = match min_dist {
                    None => Some(curr_dist),
                    Some(dist) => Some(cmp::min(dist, curr_dist)),
                };
            }
            // Update last critical point.
            last_cr = Some(idx);
        }

        // Update values.
        vals[0] = vals[1];
        vals[1] = next;
        head = node.next;
        idx += 1;
    }

    // Not enough critical points.
    if min_dist == None {
        return vec![-1, -1];
    }

    let min_dist = min_dist.unwrap();
    let max_dist = last_cr.unwrap() - first_cr.unwrap();
    return vec![min_dist, max_dist];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listnode_from() {
        let list = vec![0, 1, 2, 3];
        let node = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        assert_eq!(ListNode::from(&list), node);
    }

    macro_rules! test_nodes_between_critical_points {
        ($test: ident, $head:expr, $res:expr) => {
            #[test]
            fn $test() {
                let head = $head;
                let res = $res;
                assert_eq!(nodes_between_critical_points(head), res);
            }
        };
    }

    test_nodes_between_critical_points!(
        t1,
        ListNode::from(&vec![3, 1]),
        vec![-1, -1]
    );
    test_nodes_between_critical_points!(
        t2,
        ListNode::from(&vec![3, 1, 4]),
        vec![-1, -1]
    );
    test_nodes_between_critical_points!(
        t3,
        ListNode::from(&vec![1, 2, 3, 4, 5]),
        vec![-1, -1]
    );
    test_nodes_between_critical_points!(
        t4,
        ListNode::from(&vec![5, 3, 1, 2, 5, 1, 2]),
        vec![1, 3]
    );
    test_nodes_between_critical_points!(
        t5,
        ListNode::from(&vec![1, 3, 2, 2, 3, 2, 2, 2, 7]),
        vec![3, 3]
    );
}
