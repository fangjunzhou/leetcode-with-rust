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
}

/// Convert a i32 slice to a linked list.
///
/// * `l`: i32 slice.
pub fn to_linked_list(l: &[i32]) -> Option<Box<ListNode>> {
    // Base case.
    if l.len() == 0 {
        return None;
    }
    let mut node = ListNode::new(l[0]);
    node.next = to_linked_list(&l[1..]);
    Some(Box::new(node))
}

/// Add two positive numbers. There should not be leading zeros except zero.
///
/// * `l1`: first number linked list.
/// * `l2`: second number linked list.
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    /// Helper method to add two linked list number together.
    ///
    /// * `l1`: first number.
    /// * `l2`: second number.
    /// * `carry`: if there's a carry number.
    fn add_helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: bool,
    ) -> Option<Box<ListNode>> {
        // Base case: end of add.
        if l1 == None && l2 == None {
            return match carry {
                false => None,
                true => Some(Box::new(ListNode::new(1))),
            };
        }
        let val1 = match &l1 {
            Some(node) => node.val,
            None => 0,
        };
        let val2 = match &l2 {
            Some(node) => node.val,
            None => 0,
        };
        let carry_val = match carry {
            true => 1,
            false => 0,
        };
        let val = val1 + val2 + carry_val;
        let mut res = Box::new(ListNode::new(val % 10));
        res.next = add_helper(
            match l1 {
                Some(node) => node.next,
                None => None,
            },
            match l2 {
                Some(node) => node.next,
                None => None,
            },
            val >= 10,
        );
        Some(res)
    }

    add_helper(l1, l2, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut l = to_linked_list(&vec![1, 2, 3, 4]);
        for i in 1..=4 {
            // Check value.
            let curr = l.unwrap();
            assert_eq!(curr.val, i);
            l = curr.next;
        }
        assert_eq!(l, None);
    }

    #[test]
    fn test_linked_list_eq() {
        let l1 = to_linked_list(&vec![1, 2, 3, 4]);
        let l2 = to_linked_list(&vec![1, 2, 3, 4]);
        assert_eq!(l1, l2);
    }

    #[test]
    fn test_1() {
        let l1 = to_linked_list(&vec![2, 4, 3]);
        let l2 = to_linked_list(&vec![5, 6, 4]);
        assert_eq!(add_two_numbers(l1, l2), to_linked_list(&vec![7, 0, 8]));
    }

    #[test]
    fn test_2() {
        let l1 = to_linked_list(&vec![0]);
        let l2 = to_linked_list(&vec![0]);
        assert_eq!(add_two_numbers(l1, l2), to_linked_list(&vec![0]));
    }

    #[test]
    fn test_3() {
        let l1 = to_linked_list(&vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = to_linked_list(&vec![9, 9, 9, 9]);
        assert_eq!(
            add_two_numbers(l1, l2),
            to_linked_list(&vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
