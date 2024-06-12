advent_of_code::solution!(6);

/// Parse the coordinates from a string in the format "x,y"
fn parse_coords(input: &str) -> (usize, usize) {
    let mut parts = input.split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    (x, y)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lights = [[false; 1000]; 1000];

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let action = parts.next().unwrap();

        match action {
            "turn" => {
                let state = parts.next().unwrap();
                let (start_x, start_y) = parse_coords(parts.next().unwrap());
                let (end_x, end_y) = parse_coords(parts.last().unwrap());

                match state {
                    "on" => {
                        for x in start_x..=end_x {
                            for y in start_y..=end_y {
                                lights[x][y] = true;
                            }
                        }
                    }
                    "off" => {
                        for x in start_x..=end_x {
                            for y in start_y..=end_y {
                                lights[x][y] = false;
                            }
                        }
                    }
                    _ => {}
                }
            }
            "toggle" => {
                let (start_x, start_y) = parse_coords(parts.next().unwrap());
                let (end_x, end_y) = parse_coords(parts.last().unwrap());

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        lights[x][y] = !lights[x][y];
                    }
                }
            }
            _ => {}
        }
    });

    // Flatten the 2D array and count the number of lights that are on
    Some(lights.concat().iter().filter(|&&x| x).count())
}

pub fn part_two(input: &str) -> Option<u32> {
    // Allocate a 2D vector this time to avoid a stack overflow
    let mut lights = vec![vec![0; 1000]; 1000];

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let action = parts.next().unwrap();

        match action {
            "turn" => {
                let state = parts.next().unwrap();
                let (start_x, start_y) = parse_coords(parts.next().unwrap());
                let (end_x, end_y) = parse_coords(parts.last().unwrap());

                match state {
                    "on" => {
                        for x in start_x..=end_x {
                            for y in start_y..=end_y {
                                lights[x][y] += 1;
                            }
                        }
                    }
                    "off" => {
                        for x in start_x..=end_x {
                            for y in start_y..=end_y {
                                if lights[x][y] > 0 {
                                    lights[x][y] -= 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            "toggle" => {
                let (start_x, start_y) = parse_coords(parts.next().unwrap());
                let (end_x, end_y) = parse_coords(parts.last().unwrap());

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        lights[x][y] += 2
                    }
                }
            }
            _ => {}
        }
    });

    // Flatten the 2D array and count the number of lights that are on
    Some(lights.concat().iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let instruction_to_lights_changed = vec![
            ("turn on 0,0 through 999,999", Some(1_000_000)),
            ("toggle 0,0 through 999,0", Some(1_000)),
            ("turn off 499,499 through 500,500", Some(0)),
        ];

        for (input, expected) in instruction_to_lights_changed {
            let result = part_one(input);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_part_two() {
        let instruction_to_increase_in_brightness = vec![
            ("turn on 0,0 through 0,0", Some(1)),
            ("toggle 0,0 through 999,999", Some(2_000_000)),
        ];

        for (input, expected) in instruction_to_increase_in_brightness {
            let result = part_two(input);
            assert_eq!(result, expected);
        }
    }
}
