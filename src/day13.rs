use std::collections::HashMap;

#[aoc_generator(day13)]
pub fn day13_generator(input: &str) -> (i32, HashMap<usize, u64>) {
    let mut map: HashMap<usize, u64> = HashMap::new();
    let start: i32 = input.lines().next().unwrap().parse().unwrap();
    input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .enumerate()
        .for_each(|(idx, item)| {
            if item != "x" {
                map.insert(idx, item.parse::<u64>().unwrap());
            }
        });

    (start, map)
}

#[aoc(day13, part1)]
pub fn day13_part1(input: &(i32, HashMap<usize, u64>)) -> u64 {
    let (start, schedule) = input;
    let mut current = *start as u64;
    loop {
        let possibles: Vec<u64> = schedule.values().filter(|&&s| current % s == 0).map(|&s| s).collect();
        if possibles.len() > 0 {
            return possibles.iter().next().unwrap() * (current - *start as u64)
        }
        current += 1
    }
}

#[aoc(day13, part2)]
pub fn day13_part2(input: &(i32, HashMap<usize, u64>)) -> u64 {
    let (_, schedule) = input;
    let mut partial_solution = 0;
    let mut step: u64 = 1;

    schedule.iter().for_each(|(&idx, &item)| {
        for ts in (partial_solution..u64::MAX).step_by(step as usize) {
            if(ts + idx as u64) % item == 0 {
                partial_solution = ts;
                step *= item;
                break;
            }
        }
    });
    partial_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test2() {
        let sample = "123
3,5";
        assert_eq!(day13_part2(&day13_generator(sample)), 9);

let sample = "123
17,x,13,19";
        assert_eq!(day13_part2(&day13_generator(sample)), 3417);

        let sample = "123
67,7,59,61";
        assert_eq!(day13_part2(&day13_generator(sample)), 754018);

        let sample = "123
67,x,7,59,61";
        assert_eq!(day13_part2(&day13_generator(sample)), 779210);

        let sample = "123
67,7,x,59,61";
        assert_eq!(day13_part2(&day13_generator(sample)), 1261476);

        let sample = "123
1789,37,47,1889";
        assert_eq!(day13_part2(&day13_generator(sample)), 1202161486);
    }
}