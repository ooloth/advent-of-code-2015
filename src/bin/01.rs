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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
