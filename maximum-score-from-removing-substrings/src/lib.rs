//! [LeetCode 1717](https://leetcode.com/problems/maximum-score-from-removing-substrings/)

pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    // Construct stack.
    let stack: Vec<char> = s.chars().collect();

    #[derive(Debug)]
    enum RemoveType {
        BA,
        AB,
    }

    /// Remove all the ab/ba's in the stack and return the score.
    fn remove(stack: Vec<char>, score: i32, t: RemoveType) -> (Vec<char>, i32) {
        let mut res = 0;
        let mut working_stack: Vec<char> = Vec::new();
        // Pop the stack.
        for ch in stack.into_iter().rev() {
            // Pop the top of the working stack.
            let top = match working_stack.pop() {
                Some(top) => top,
                None => {
                    working_stack.push(ch);
                    continue;
                }
            };
            // Match.
            let mat = match t {
                RemoveType::AB => ch == 'a' && top == 'b',
                RemoveType::BA => ch == 'b' && top == 'a',
            };
            if mat {
                res += score;
                continue;
            }
            // Put back into the working stack.
            working_stack.push(top);
            working_stack.push(ch);
        }
        // Recover the original stack.
        let stack = working_stack.into_iter().rev().collect();
        return (stack, res);
    }

    // Remove.
    let mut res = 0;
    match x.cmp(&y) {
        std::cmp::Ordering::Greater => {
            let (stack, score) = remove(stack, x, RemoveType::AB);
            res += score;
            let (_, score) = remove(stack, y, RemoveType::BA);
            res += score;
        }
        _ => {
            let (stack, score) = remove(stack, y, RemoveType::BA);
            res += score;
            let (_, score) = remove(stack, x, RemoveType::AB);
            res += score;
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("cdbcbbaaabab");
        let x = 4;
        let y = 5;
        assert_eq!(maximum_gain(s, x, y), 19);
    }

    #[test]
    fn test_2() {
        let s = String::from("aabbaaxybbaabb");
        let x = 5;
        let y = 4;
        assert_eq!(maximum_gain(s, x, y), 20);
    }

    #[test]
    fn test_3() {
        let s = String::from("a");
        let x = 5;
        let y = 4;
        assert_eq!(maximum_gain(s, x, y), 0);
    }
}
