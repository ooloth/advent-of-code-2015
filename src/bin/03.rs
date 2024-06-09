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

pub fn part_two(input: &str) -> Option<u32> {
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    coords.insert((0, 0));

    let mut santa = (0, 0);
    let mut robot = (0, 0);

    input.chars().enumerate().for_each(|(i, c)| {
        let mover = if i % 2 == 0 { &mut santa } else { &mut robot };

        match c {
            '^' => mover.1 += 1,
            'v' => mover.1 -= 1,
            '<' => mover.0 -= 1,
            '>' => mover.0 += 1,
            _ => (),
        }

        coords.insert(*mover);
    });

    Some(coords.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let directions_to_num_houses_visited =
            vec![(">", Some(2)), ("^>v<", Some(4)), ("^v^v^v^v^v", Some(2))];

        for (input, expected) in directions_to_num_houses_visited {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let directions_to_num_houses_visited =
            vec![("^v", Some(3)), ("^>v<", Some(3)), ("^v^v^v^v^v", Some(11))];

        for (input, expected) in directions_to_num_houses_visited {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
