advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut dimensions = line.split('x').map(|s| s.parse::<u32>().unwrap());

                let l = dimensions.next().unwrap();
                let w = dimensions.next().unwrap();
                let h = dimensions.next().unwrap();

                let lw = l * w;
                let wh = w * h;
                let hl = h * l;

                let smallest_area = lw.min(wh).min(hl);

                2 * (lw + wh + hl) + smallest_area
            })
            .sum(),
    )
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
            ("2x3x4", Some(58)),
            ("1x1x10", Some(43)),
            ("2x3x4\n1x1x10", Some(58 + 43)),
        ];

        for (input, expected) in examples {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let examples = vec![
            ("1-3 a: abcde", Some(1)),
            ("1-3 b: cdefg", Some(0)),
            ("2-9 c: ccccccccc", Some(1)),
        ];

        for (input, expected) in examples {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
