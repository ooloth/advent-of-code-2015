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

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let chars: Vec<char> = line.chars().collect();
                let len = chars.len();

                // Check for the same letter sandwiching another letter
                let has_letter_sandwich = chars.windows(3).any(|w| w[0] == w[2]);

                // If no letter sandwich is found, this string is not nice
                if !has_letter_sandwich {
                    return false;
                }

                // Store pairs and their indices so we can look for repeating, non-overlapping pairs
                let mut pairs_with_indices: Vec<((char, char), usize)> = Vec::new();
                let mut has_repeating_non_overlapping_pair = false;

                // Iterate over all pairs of characters in the string
                for i in 0..len - 1 {
                    let pair = (chars[i], chars[i + 1]);

                    // Check if we already found this pair at least two indices ago (non-overlapping)
                    match pairs_with_indices
                        .iter()
                        .find(|&&(p, index)| p == pair && i - index > 1)
                    {
                        Some(_) => {
                            // If so, this string is nice, so we can break and return early
                            has_repeating_non_overlapping_pair = true;
                            break;
                        }
                        _ => {
                            // Otherwise, add this pair to the list of pairs and their indices and continue
                            pairs_with_indices.push((pair, i));
                        }
                    }
                }

                // Return the result
                has_repeating_non_overlapping_pair
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

    #[test]
    fn test_part_two() {
        let strings_to_num_nice = vec![
            ("qjhvhtzxzqqjkmpb", Some(1)), // has a pair that repeats (qj) + letters that repeat with one letter between them (hvh, zxz)
            ("xxyxx", Some(1)), // has a pair that repeats (xx) + a letter that repeats with one letter between (xyx)
            ("uurcxstgmygtbstg", Some(0)), // has a pair that repeats (tg), but no letter that repeats with one letter between
            ("ieodomkazucvgmuy", Some(0)), // has a letter that repeats with one letter between (odo), but no pair that repeats
            (
                "qjhvhtzxzqqjkmpb\nxxyxx\nuurcxstgmygtbstg\nieodomkazucvgmuy",
                Some(1 + 1 + 0 + 0),
            ),
        ];

        for (input, expected) in strings_to_num_nice {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
