advent_of_code::solution!(4);

// see: https://docs.rs/md-5/latest/md5/
use md5::{Digest, Md5};

fn lowest_num_that_produces_hash_with_x_leading_zeroes(
    input: &str,
    num_leading_zeroes: usize,
) -> Option<u32> {
    Some(
        (0..u32::MAX)
            .into_iter()
            .find(|num| {
                let combined = format!("{input}{num}");
                let hash = Md5::digest(combined.as_bytes());
                let hex_hash = format!("{:x}", hash);
                hex_hash.starts_with(&"0".repeat(num_leading_zeroes))
            })
            .unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    lowest_num_that_produces_hash_with_x_leading_zeroes(input, 5)
}

pub fn part_two(input: &str) -> Option<u32> {
    lowest_num_that_produces_hash_with_x_leading_zeroes(input, 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let secret_key_to_number = vec![("abcdef", Some(609043)), ("pqrstuv", Some(1048970))];

        for (input, expected) in secret_key_to_number {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {}
}
