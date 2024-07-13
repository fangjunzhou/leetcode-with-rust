//! [LeetCode 2751](https://leetcode.com/problems/robot-collisions/)

pub fn survived_robots_healths(
    positions: Vec<i32>,
    healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    #[derive(Debug, PartialEq)]
    /// Bot moving direction.
    enum Direction {
        Left,
        Right,
    }
    #[derive(Debug)]
    /// A bot instance.
    ///
    /// * `id`:
    /// * `position`:
    /// * `health`:
    /// * `direction`:
    struct Bot {
        id: usize,
        position: i32,
        health: i32,
        direction: Direction,
    }

    // Construct bots.
    let mut bots: Vec<Bot> = Vec::new();
    for i in 0..positions.len() {
        let direction = match directions[i..=i].chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        };
        bots.push(Bot {
            id: i,
            position: positions[i],
            health: healths[i],
            direction,
        });
    }
    // Sort all the bots based on position.
    bots.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

    // Working stack.
    let mut working_stack: Vec<Bot> = Vec::new();
    while let Some(mut bot) = bots.pop() {
        loop {
            // Get the top of the working stack.
            let mut top = match working_stack.pop() {
                Some(bot) => bot,
                None => {
                    working_stack.push(bot);
                    break;
                }
            };

            // If there's a battle.
            if bot.direction == Direction::Right
                && top.direction == Direction::Left
            {
                match bot.health.cmp(&top.health) {
                    std::cmp::Ordering::Greater => {
                        // Left win, continue poping the top.
                        bot.health -= 1;
                    }
                    std::cmp::Ordering::Less => {
                        // Right win, decrease health and put back to stack.
                        top.health -= 1;
                        working_stack.push(top);
                        break;
                    }
                    std::cmp::Ordering::Equal => {
                        // Drop both.
                        break;
                    }
                }
            } else {
                working_stack.push(top);
                working_stack.push(bot);
                break;
            }
        }
    }

    // Sort get the result health sorted by id.
    working_stack.sort_by_key(|a| a.id);
    let res: Vec<i32> = working_stack.iter().map(|a| a.health).collect();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let positions = vec![5, 4, 3, 2, 1];
        let healths = vec![2, 17, 9, 15, 10];
        let directions = String::from("RRRRR");
        assert_eq!(
            survived_robots_healths(positions, healths, directions),
            vec![2, 17, 9, 15, 10]
        );
    }

    #[test]
    fn test_2() {
        let positions = vec![3, 5, 2, 6];
        let healths = vec![10, 10, 15, 12];
        let directions = String::from("RLRL");
        assert_eq!(
            survived_robots_healths(positions, healths, directions),
            vec![14]
        );
    }

    #[test]
    fn test_3() {
        let positions = vec![1, 2, 5, 6];
        let healths = vec![10, 10, 11, 11];
        let directions = String::from("RLRL");
        assert_eq!(
            survived_robots_healths(positions, healths, directions),
            vec![]
        );
    }

    #[test]
    fn test_4() {
        let positions = vec![4, 3, 2, 1];
        let healths = vec![17, 9, 15, 10];
        let directions = String::from("RRLL");
        assert_eq!(
            survived_robots_healths(positions, healths, directions),
            vec![17, 9, 15, 10]
        );
    }
}
