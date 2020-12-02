use std::{cmp::Ordering, collections::BTreeSet};
use itertools::Itertools;

const TARGET: i32 = 2020;

#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1, BTreeSet)]
pub fn part1(inputs: &[i32]) -> i32 {
    let mut seen = BTreeSet::new();

    for input in inputs {
        let remainder = TARGET - *input;
        if seen.contains(&remainder) {
            return remainder * input;
        }

        seen.insert(*input);
    }
    unreachable!()
}

#[aoc(day1, part1, itertools)]
pub fn part1_iter(inputs: &[i32]) -> i32 {
    return inputs
        .iter()
        .combinations(2)
        .find(|x| x[0] + x[1] == TARGET)
        .map(|x| x[0] * x[1])
        .unwrap();
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    let mut inputs = inputs.to_vec();
    inputs.sort_unstable();
    let len = inputs.len();

    for (i, a) in inputs[0..(len - 2)].iter().enumerate() {
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            let b = inputs[left];
            let c = inputs[right];
            let sum = a + b + c;

            match sum.cmp(&TARGET) {
                Ordering::Less => left += 1,
                Ordering::Equal => return a * b * c,
                Ordering::Greater => right -= 1,
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2, cartesian)]
pub fn part2_cartesian(inputs: &[i32]) -> i32 {
    let ((a, b), c) = inputs
        .iter()
        .cartesian_product(inputs)
        .cartesian_product(inputs)
        .find(|((a, b), c)| *a + *b + *c == TARGET)
        .expect("Query was not found");

    return a * b * c;
}

#[aoc(day1, part2, itertools)]
pub fn part2_iter(inputs: &[i32]) -> i32 {
    return inputs
        .iter()
        .combinations(3)
        .find(|x| x[0] + x[1] + x[2] == TARGET)
        .map(|x| x[0] * x[1] * x[2])
        .unwrap();

}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 1721 * 299);
        assert_eq!(part1_iter(&SAMPLE), 1721 * 299);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 979 * 366 * 675);
        assert_eq!(part2_iter(&SAMPLE), 979 * 366 * 675);
        assert_eq!(part2_cartesian(&SAMPLE), 979 * 366 * 675);
    }
}