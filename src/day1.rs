use std::{collections::HashMap, iter::zip};

pub fn part1(input: String) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let line_items: Vec<&str> = line.split_whitespace().collect();
        left.push(line_items[0].to_string());
        right.push(line_items[1].to_string());
    }

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(left_str, right_str)| {
            let left_num: i32 = left_str.parse().unwrap();
            let right_num: i32 = right_str.parse().unwrap();
            (left_num - right_num).abs()
        })
        .sum()
}

pub fn part2(input: String) -> i32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let line_items: Vec<&str> = line.split_whitespace().collect();
        left.push(line_items[0].to_string());
        right.push(line_items[1].to_string());
    }

    let mut right_number_occurences = HashMap::new();
    for num_str in right {
        let num: i32 = num_str.parse().unwrap();
        *right_number_occurences.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|num_str| {
            let num: i32 = num_str.parse().unwrap();
            num * right_number_occurences.get(&num).or(Some(&0)).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use advent::get_day_input;

    use super::*;

    #[test]
    fn test_day1_part1() {
        let input = get_day_input(1);
        let solution = part1(input);
        println!("{:?}", solution);
    }

    #[test]
    fn test_day1_part2() {
        let input = get_day_input(1);
        let solution = part2(input);
        println!("{:?}", solution);
    }
}
