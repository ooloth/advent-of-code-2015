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

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut dimensions: Vec<u32> =
                    line.split('x').map(|s| s.parse::<u32>().unwrap()).collect();

                dimensions.sort();

                2 * (dimensions[0] + dimensions[1]) + dimensions[0] * dimensions[1] * dimensions[2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let dimensions_to_square_feet_of_wrapping_paper = vec![
            ("2x3x4", Some(58)),
            ("1x1x10", Some(43)),
            ("2x3x4\n1x1x10", Some(58 + 43)),
        ];

        for (input, expected) in dimensions_to_square_feet_of_wrapping_paper {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let dimensions_to_feet_of_ribbon = vec![
            ("2x3x4", Some((2 + 2 + 3 + 3) + (2 * 3 * 4))),   // 34
            ("1x1x10", Some((1 + 1 + 1 + 1) + (1 * 1 * 10))), // 14
            ("2x3x4\n1x1x10", Some(34 + 14)),                 // 48
        ];

        for (input, expected) in dimensions_to_feet_of_ribbon {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
