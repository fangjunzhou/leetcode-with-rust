pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn solve(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        // Recursive cases.
        for i in 0..candidates.len() {
            // Skip if the candidate is the same.
            if i > 0 && candidates[i - 1] == candidates[i] {
                continue;
            }
            let curr = candidates[i];
            let new_res = solve(&candidates[i + 1..], target - curr);
            for mut comb in new_res {
                comb.insert(0, curr);
                res.push(comb);
            }
        }
        // Base case.
        for num in candidates {
            if *num == target {
                res.push(vec![target]);
                break;
            }
        }
        return res;
    }
    // Sort the candidates.
    let mut candidates = candidates.clone();
    candidates.sort();
    return solve(&candidates, target);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        assert_eq!(
            combination_sum2(candidates, target),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test_2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        assert_eq!(
            combination_sum2(candidates, target),
            vec![vec![1, 2, 2], vec![5]]
        );
    }

    #[test]
    fn test_3() {
        let mut candidates = Vec::new();
        for i in 1..=100 {
            candidates.push(i);
        }
        let target = 5050;
        assert_eq!(combination_sum2(candidates, target), {
            let mut res = Vec::new();
            for i in 1..=100 {
                res.push(i);
            }
            vec![res]
        });
    }
}
