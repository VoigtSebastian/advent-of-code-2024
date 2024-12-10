advent_of_code::solution!(2);

fn are_indices_valid(should_increase: bool, numbers: &[i32], i: usize, j: usize) -> bool {
    let difference = (numbers[i] - numbers[j]).abs();

    let difference_in_range = difference >= 1 && difference <= 3;
    let increase_correct = should_increase == (numbers[i] < numbers[j]);

    increase_correct && difference_in_range
}

fn parse_level(level: &str) -> Vec<i32> {
    level
        .split(" ")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn check_level(numbers: &[i32]) -> bool {
    if numbers.len() <= 1 {
        return true;
    }

    let is_increasing = numbers[0] < numbers[1];
    for i in 0..(numbers.len() - 1) {
        if !are_indices_valid(is_increasing, numbers, i, i + 1) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_levels = 0;

    for level in input.lines() {
        let numbers = parse_level(level);
        if check_level(&numbers) {
            safe_levels += 1;
        }
    }
    Some(safe_levels)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_levels = 0;

    for level in input.lines() {
        let numbers = parse_level(level);

        if check_level(&numbers) {
            safe_levels += 1;
            continue;
        }

        for n in 0..level.len() {
            // Create vector without index n
            let slice: Vec<i32> = numbers
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, v)| *v)
                .collect();

            // Check if the level is ok without index n
            if check_level(&slice) {
                safe_levels += 1;
                break;
            }
        }
    }
    return Some(safe_levels);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_single_level_safe() {
        let input = "1 2 3 4 5";
        assert_eq!(part_two(input), Some(1));
    }

    #[test]
    fn test_part_two_single_level_unsafe() {
        // If you take out the 6 (a safe value) the level becomes safe
        // 1 2 4 5
        let input = "1 2 6 4 5";
        assert_eq!(part_two(input), Some(1));
    }

    #[test]
    fn test_part_two_single_level_with_dampener() {
        let input = "1 2 6 7 8";
        assert_eq!(part_two(input), Some(0));
    }
}
