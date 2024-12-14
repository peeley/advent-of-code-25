pub fn part1(input: String) -> usize {
    let lines: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|str| str.parse().unwrap())
            .collect()
    }).collect();

    lines.iter().filter(|line| {
        let increasing = line.windows(2).all(|nums| {
            nums[0] < nums[1] && (nums[1] - nums[0]) <= 3
        });

        let decreasing = line.windows(2).all(|nums| {
            nums[0] > nums[1] && (nums[0] - nums[1]) <= 3
        });

        increasing || decreasing
    }).count()
}

#[cfg(test)]
mod tests {
    use advent::get_day_input;

    use super::*;

    #[test]
    fn test_day2_part1() {
        let input = get_day_input(2);
        let solution = part1(input);
        println!("{:?}", solution);
    }
}
