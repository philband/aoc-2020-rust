use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn day15_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn day15_part1(input: &[usize]) -> usize {
    get_nth_map(input, 2020)
}

#[aoc(day15, part2)]
pub fn day15_part2(input: &[usize]) -> usize {
    get_nth_map(input, 30000000)
}

pub fn get_nth(input: &[u64], n: usize) -> u64 {
    let mut map: HashMap<usize, u64> = HashMap::new();

    input.iter().enumerate().for_each(|(k, &v)| { map.insert(k, v); });

    for i in input.len()..n {
        let &last = map.get(&(map.len()-1)).unwrap();
        let mut spoken: Vec<u64> = map.iter().filter(|(&_k, &v)| v == last).map(|(&k, _)| k as u64).collect();
        spoken.sort_unstable();
        if spoken.len() < 2 {
            map.insert(i, 0);
        } else {
            map.insert(i, spoken[spoken.len()-1] - spoken[spoken.len()-2]);
        }
    }
    *map.get(&(n-1)).unwrap()
}

pub fn get_nth_map(input: &[usize], n: usize) -> usize {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    input
        .iter()
        .enumerate()
        .for_each(|(k, &v)| {
            if map.get(&v).is_some() {
                map.get_mut(&v).unwrap().push(k);
            } else {
                let mut newv: Vec<usize> = Vec::new();
                newv.push(k);
                map.insert(v, newv);
            }
             });

    let mut last = input[input.len()-1];
    let mut spoken: usize;
    for i in input.len()..n {
        //println!("last: {}, spoken: {}", last, spoken);
        //for (&k, v) in map.iter() {
        //    print!("{}|{} => [ ", i, k);
        //    for i in v.iter() {
        //        print!("{} ", i)
        //    }
        //    println!("]")
        //}
        if map.get(&last).is_some() {
            let prev = map.get(&last).unwrap();
            if prev.len() >= 2 {
                spoken = prev[prev.len()-1] - prev[prev.len()-2];
            } else {
                spoken = 0;
            }

        } else {
            spoken = 0
        }

        if map.get(&spoken).is_some() {
            map.get_mut(&spoken).unwrap().push(i);
        } else {
            map.insert(spoken, vec![i]);
        }
        //println!("last: {}, spoken: {}", last, spoken);
        //println!("==================================================");
        last = spoken;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let data = day15_generator("0,3,6");
        assert_eq!(get_nth_map(&data, 4), 0);
        assert_eq!(get_nth_map(&data, 5), 3);
        assert_eq!(get_nth_map(&data, 6), 3);
        assert_eq!(get_nth_map(&data, 7), 1);
        assert_eq!(get_nth_map(&data, 8), 0);
        assert_eq!(get_nth_map(&data, 9), 4);
        assert_eq!(get_nth_map(&data, 10), 0);
        assert_eq!(day15_part1(&day15_generator("0,3,6")), 436);
        assert_eq!(day15_part1(&day15_generator("1,3,2")), 1);
        assert_eq!(day15_part1(&day15_generator("2,1,3")), 10);
        assert_eq!(day15_part1(&day15_generator("1,2,3")), 27);
        assert_eq!(day15_part1(&day15_generator("2,3,1")), 78);
        assert_eq!(day15_part1(&day15_generator("3,2,1")), 438);
        assert_eq!(day15_part1(&day15_generator("3,1,2")), 1836);
    }
}