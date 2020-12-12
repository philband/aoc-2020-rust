use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn day10_generator(input: &str) -> Vec<usize> {
    let mut v = input
        .lines()
        .flat_map(|line| line.parse())
        .collect::<Vec<usize>>();
    v.sort();
    v.insert(0, 0);
    v.push(v[v.len() - 1] + 3);
    v
}

#[aoc(day10, part1)]
pub fn day10_part1(input: &[usize]) -> u64 {
    let diffs = input[..].windows(2).map(|c| c[1] - c[0]).fold(HashMap::new(), |mut map, diff| {
        *map.entry(diff).or_insert(0) += 1;
        map
    });
    diffs[&1] * diffs[&3]
}

#[aoc(day10, part2)]
pub fn day10_part2(input: &[usize]) -> u64 {
    let input = input.to_vec();
    let max  = input[input.len() - 1];
    let mut dp = vec![0u64; max + 1];
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