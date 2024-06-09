advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let has_forbidden = ["ab", "cd", "pq", "xy"]
                    .iter()
                    .any(|forbidden| line.contains(forbidden));

                if has_forbidden {
                    return false;
                }

                let num_vowels = line.chars().filter(|c| "aeiou".contains(*c)).count();
                let has_double = line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b);

                num_vowels >= 3 && has_double
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let strings_to_num_nice = vec![
            ("ugknbfddgicrmopn", Some(1)), // 3 vowels, a double letter, and no forbidden strings
            ("aaa", Some(1)),              // 3 vowels, a double letter, and no forbidden strings
            ("jchzalrnumimnmhp", Some(0)), // no double
            ("haegwjzuvuyypxyu", Some(0)), // "xy" forbidden
            ("dvszwmarrgswjxmb", Some(0)), // no double
            (
                "ugknbfddgicrmopn\naaa\njchzalrnumimnmhp\nhaegwjzuvuyypxyu\ndvszwmarrgswjxmb",
                Some(1 + 1 + 0 + 0 + 0),
            ),
        ];

        for (input, expected) in strings_to_num_nice {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

}
