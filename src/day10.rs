use std::collections::HashMap;
use countmap::CountMap;
use itertools::Itertools;
use std::ops::Deref;

#[aoc_generator(day10)]
pub fn day10_generator(input: &str) -> Vec<i32> {
    let mut vec: Vec<i32> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    vec.sort_unstable();
    vec
}

#[aoc(day10, part1)]
pub fn day10_part1(input: &[i32]) -> u64 {
    let mut cm: CountMap<i32> = CountMap::new();
    for i in 0..4 {
        cm.insert_or_increment(i);
    }
    for i in 0..input.len()-1 {
        let diff: i32 = (&input[i] - &input[i+1]).abs();
        &cm.insert_or_increment(diff);
    }
    return cm.get_count(&1).unwrap() * cm.get_count(&3).unwrap()
}

#[aoc(day10, part2)]
pub fn day10_part2(input: &[i32]) -> u64 {
    let mut input = input.clone().to_vec();
    input.insert(0, 0);
    input.push(input[input.len() - 1] + 3);
    let input: Vec<usize> = input.iter().map(|x| *x as usize).collect();

    let max: usize  = input[input.len() - 1];
    let mut dp = vec![0u64; max as usize + 1];
    for x in input {
        match x {
            0 => dp[x] = 1,
            1 => dp[x] += dp[x - 1],
            2 => dp[x] += dp[x - 1] + dp [x - 2],
            _ => dp[x] += dp[x - 1] + dp [x - 2] + dp [x - 3],
        }
    }
    dp[max]
}

pub fn possible(input: &[i32]) -> bool {
    false
}

pub fn is_possible(input: &[i32]) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample ="28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        let gen = day10_generator(sample);

        //gen.iter().for_each(|x| println!("{}", x));

        assert_eq!(day10_part1(&gen), 220);
        assert_eq!(day10_part2(&gen), 19208);
        //assert_eq!(is_possible(gen.as_slice()), true);


    }
}