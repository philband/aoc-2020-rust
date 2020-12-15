#[aoc_generator(day9)]
pub fn day9_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn day9_part1(input: &Vec<usize>) -> usize {
    for i in 25..input.len() {
        let target = input[i];
        let haystack = &input[i - 25..i];
        if !haystack
            .iter()
            .any(|x| haystack.iter().any(|y| x != y && x + y == target))
        {
            return input[i];
        }
    }
    0
}

#[aoc(day9, part2)]
pub fn day9_part2(input: &Vec<usize>) -> usize {
    let magic_invalid_number = day9_part1(&input);
    for start in 0..input.len() {
        let mut sum = 0;
        for i in start..input.len() {
            sum += input[i];
            if sum > magic_invalid_number {
                break;
            } else if sum == magic_invalid_number {
                return input[start..i].iter().min().unwrap()
                    + input[start..i].iter().max().unwrap();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test0() {
        let sample = "1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
49
100
50";
        let generated = day9_generator(&sample);
        assert_eq!(day9_part1(&generated), 100)
    }

    #[test]
    pub fn test1() {
        let sample = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let generated = day9_generator(&sample);
        assert_eq!(day9_part1(&generated), 127)
    }
}
