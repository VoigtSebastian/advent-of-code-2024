advent_of_code::solution!(1);

fn parse_string_to_separate_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in input.lines() {
        let mut parts = line.split("   ");
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            if let (Ok(left), Ok(right)) =
                (first.trim().parse::<i32>(), second.trim().parse::<i32>())
            {
                left_values.push(left);
                right_values.push(right);
            }
        }
    }

    (left_values, right_values)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_string_to_separate_lists(input);

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right)
            .map(|(l, r)| (l - r).unsigned_abs())
            .sum(),
    )
}

// Why does this template include two parts again?
pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
