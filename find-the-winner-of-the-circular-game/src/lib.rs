//! [LeetCode 1823](https://leetcode.com/problems/find-the-winner-of-the-circular-game/)

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut friends: Vec<i32> = (0..n).into_iter().collect();
    // Current friend pointer.
    let mut curr = 0;
    while friends.len() != 1 {
        // Find the friend to kick.
        curr = (curr + k - 1) % friends.len() as i32;
        friends.remove(curr as usize);
        // Start a new round.
        curr = curr % friends.len() as i32;
    }
    return friends[0] + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_winner() {
        assert_eq!(find_the_winner(5, 2), 3);
        assert_eq!(find_the_winner(6, 5), 1);
        assert_eq!(find_the_winner(1, 2), 1);
        assert_eq!(find_the_winner(500, 1), 500);
    }
}
