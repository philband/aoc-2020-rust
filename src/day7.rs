use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug,Eq, PartialEq, Hash)]
pub struct BagContains {
    bag_name: String,
    bag_count: usize,
}

#[aoc_generator(day7)]
pub fn day7_generator(input: &str) -> HashMap<String, HashSet<BagContains>> {
    let mut bag_map: HashMap<String, HashSet<BagContains>> = HashMap::new();
    input.lines().for_each(|line| {
        let parts: Vec<String> = line.split("contain ").map(|x| x.to_string()).collect();
        let bag_filter = Regex::new(r"^(\w+\s\w+)\sbags").unwrap();
        let colors_filter = Regex::new(r"(\d+)\s(\w+\s\w+)\sbags?[\.,]").unwrap();
        let name = &bag_filter.captures(&parts[0]).unwrap()[1];
        bag_map.insert(
            String::from(name),
            colors_filter
                .captures_iter(&parts[1])
                .map(|cap| BagContains {
                    bag_name: String::from(cap.get(2).unwrap().as_str()),
                    bag_count: cap.get(1).unwrap().as_str().parse().unwrap(),
                })
                .collect(),
        );
    });
    bag_map
}

#[aoc(day7, part1)]
pub fn day7_part1(input: &HashMap<String, HashSet<BagContains>>) -> usize {
    let mut container_list: HashSet<String> = HashSet::new();
    container_list.insert(String::from("shiny gold"));
    let mut size_before: usize = 0;

    while container_list.len() != size_before {
        size_before = container_list.len();
        for needle in container_list.clone() {
            input
                .iter()
                .filter(|(_k, v)| v.iter().any(|vals| vals.bag_name.contains(&needle)))
                .for_each(|(k, _v)| {
                    container_list.insert(k.clone());
                });
        }
    }
    container_list.len() - 1
}

pub fn descend(input: &HashMap<String, HashSet<BagContains>>, name: &String, multi: usize) -> usize {
    match input.get(name) {
        Some(v) => {
            match v.len() {
                0 => multi,
                _ => {
                    v.iter().map(|bag| descend(&input, &bag.bag_name, bag.bag_count) * multi).sum::<usize>() + multi
                }
            }
        },
        _ => 0
    }
}

#[aoc(day7, part2)]
pub fn day7_part2(input: &HashMap<String, HashSet<BagContains>>) -> usize {
    descend(&input, &String::from("shiny gold"), 1) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let generated = day7_generator(&sample);

        assert_eq!(day7_part1(&generated), 4);
    }
    #[test]
    pub fn test2() {
        let sample = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let generated = day7_generator(&sample);
        assert_eq!(day7_part2(&generated), 32);

        let sample = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let generated = day7_generator(&sample);
        assert_eq!(day7_part2(&generated), 126);
    }
}
