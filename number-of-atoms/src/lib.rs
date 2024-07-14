//! [LeetCode 726](https://leetcode.com/problems/number-of-atoms/)

pub fn count_of_atoms(formula: String) -> String {
    use std::collections::HashMap;
    /// Solve the substring.
    ///
    /// * `s`: substring slice.
    fn solve(s: &str) -> HashMap<String, u32> {
        dbg!(s);
        // Base case
        if s.len() == 0 {
            return HashMap::new();
        }
        // Find the current item.
        let mut start_new = false;
        let mut map: HashMap<String, u32> = HashMap::new();
        let mut atom: Option<String> = None;
        let mut count: Option<u32> = None;
        let mut next_idx = 0;

        let mut chars = s.chars().enumerate();
        while let Some((mut idx, ch)) = chars.next() {
            match ch {
                '0'..='9' => {
                    // Increase count.
                    let num = ch as u32 - '0' as u32;
                    count = match count {
                        Some(cnt) => Some(cnt * 10 + num),
                        None => Some(num),
                    };
                }
                'A'..='Z' => {
                    // Set the atom name or start a new one.
                    if start_new {
                        break;
                    }
                    atom = Some(String::from(ch));
                    start_new = true;
                }
                'a'..='z' => {
                    // Set the atom name.
                    atom.as_mut().unwrap().push(ch);
                }
                '(' => {
                    // Solve the sub array.
                    if start_new {
                        break;
                    }
                    let mut depth = 1;
                    while let Some((i, ch)) = chars.next() {
                        idx = i;
                        match ch {
                            '(' => depth += 1,
                            ')' => depth -= 1,
                            _ => {}
                        }
                        if depth == 0 {
                            map = solve(&s[1..i]);
                            break;
                        }
                    }
                    start_new = true;
                }
                _ => unreachable!(),
            }
            next_idx = idx;
        }
        let count = count.unwrap_or(1);
        if let Some(atom) = atom {
            map.entry(atom).and_modify(|c| *c += count).or_insert(count);
        } else {
            for c in map.values_mut() {
                *c = *c * count;
            }
        }
        // Solve the rest of the string.
        for (atom, count) in solve(&s[next_idx + 1..]) {
            map.entry(atom).and_modify(|c| *c += count).or_insert(count);
        }

        return map;
    }

    // Solve the entire string.
    let map = solve(&formula);
    let mut sorted_map: Vec<(&String, &u32)> = map.iter().collect();
    sorted_map.sort_by_key(|i| i.0);
    let mut res = String::new();
    let _ = sorted_map
        .iter()
        .map(|(atom, count)| {
            res.push_str(atom);
            if **count > 1 {
                res.push_str(&count.to_string());
            }
        })
        .collect::<Vec<_>>();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let formula = String::from("H2O");
        assert_eq!(count_of_atoms(formula), String::from("H2O"));
    }

    #[test]
    fn test_2() {
        let formula = String::from("Mg(OH)2");
        assert_eq!(count_of_atoms(formula), String::from("H2MgO2"));
    }

    #[test]
    fn test_3() {
        let formula = String::from("K4(ON(SO3)2)2");
        assert_eq!(count_of_atoms(formula), String::from("K4N2O14S4"));
    }

    #[test]
    fn test_4() {
        let formula = String::from("(H2O)");
        assert_eq!(count_of_atoms(formula), String::from("H2O"));
    }

    #[test]
    fn test_5() {
        let formula = String::from("(H2O2)(H2O2)");
        assert_eq!(count_of_atoms(formula), String::from("H4O4"));
    }

    #[test]
    fn test_6() {
        let formula = String::from("(H2O2)()");
        assert_eq!(count_of_atoms(formula), String::from("H2O2"));
    }

    #[test]
    fn test_7() {
        let formula = String::from("Heiu19(JuO4)5");
        assert_eq!(count_of_atoms(formula), String::from("Heiu19Ju5O20"));
    }
}
