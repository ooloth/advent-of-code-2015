advent_of_code::solution!(3);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    coords.insert((0, 0));

    let mut x = 0;
    let mut y = 0;

    input.chars().for_each(|c| {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => (),
        }
        coords.insert((x, y));
    });

    Some(coords.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let directions_to_number_of_houses_visited =
            vec![(">", Some(2)), ("^>v<", Some(4)), ("^v^v^v^v^v", Some(2))];

        for (input, expected) in directions_to_number_of_houses_visited {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let examples = vec![("abc", Some(1)), ("def", Some(1)), ("ghi", Some(1))];

        for (input, expected) in examples {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
