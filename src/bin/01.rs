advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |acc, char| {
        acc + match char {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .chars()
            .scan(0, |floor, char| {
                *floor += match char {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                Some(*floor)
            })
            .position(|floor| floor == -1)
            .unwrap() as i32
            + 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let examples = vec![
            ("(())", 0),     // 0
            ("()()", 0),     // 0
            ("(((", 3),      // 3
            ("(()(()(", 3),  // 3
            ("))(((((", 3),  // 3
            ("())", -1),     // -1
            ("))(", -1),     // -1
            (")))", -3),     // -3
            (")())())", -3), // -3
        ];

        for (input, expected) in examples {
            let result = part_one(input);
            assert_eq!(result, Some(expected));
        }
    }

    #[test]
    fn test_part_two() {
        let examples = vec![
            (")", 1),     // 1
            ("()())", 5), // 5
        ];

        for (input, expected) in examples {
            let result = part_two(input);
            assert_eq!(result, Some(expected));
        }
    }
}
