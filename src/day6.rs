use std::collections::HashSet;
use countmap::CountMap;
use std::convert::TryInto;

#[aoc_generator(day6)]
pub fn day6_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

#[aoc(day6, part1)]
pub fn day6_part1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut set = HashSet::new();
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .for_each(|c| { set.insert(c); });
            set.len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn day6_part2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|group| {
            let mut cm: CountMap<char> = CountMap::new();
            let individual_answers: Vec<String> = group.split('\n').map(|s| s.to_string()).collect();
            individual_answers
                .iter()
                .for_each(|answers| {
                    answers
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .for_each(|c| { cm.insert_or_increment(c); })
                });
            cm.iter().map(|(_, v)| if *v == individual_answers.len().try_into().unwrap() { 1 } else { 0 }).sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample =
        "abc

a
b
c

ab
ac

a
a
a
a

b";
        let generated = day6_generator(&sample);
        assert_eq!(day6_part1(&generated), 11);
    }
    #[test]
    pub fn test2() {
        let sample =
            "abc

a
b
c

ab
ac

a
a
a
a

b";
        let generated = day6_generator(&sample);
        assert_eq!(day6_part2(&generated), 6);
    }
}