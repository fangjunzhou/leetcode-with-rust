//! [LeetCode 2582](https://leetcode.com/problems/pass-the-pillow/description/)

pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    // Direction.
    match (time / (n - 1)) % 2 {
        0 => {
            // Forward.
            return (time % (n - 1)) + 1;
        }
        1 => {
            // Backward.
            return n - (time % (n - 1));
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_pass_the_pillow {
        ($func:ident, $n:expr, $time:expr, $ref:expr) => {
            #[test]
            fn $func() {
                assert_eq!(pass_the_pillow($n, $time), $ref);
            }
        };
    }

    test_pass_the_pillow!(t1, 4, 5, 2);
    test_pass_the_pillow!(t2, 3, 2, 3);
    test_pass_the_pillow!(t3, 2, 1, 2);
    test_pass_the_pillow!(t4, 100, 896, 95);
}
