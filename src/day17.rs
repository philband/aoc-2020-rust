use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;
use std::mem::swap;

const CYCLES: i32 = 6;

#[aoc_generator(day17)]
pub fn day17_generator(input: &str) -> HashSet<(i32, i32, i32, i32)> {
    let mut set: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let xdim = ((input.lines().enumerate().last().unwrap().0 as i32) / 2);
    let ydim = ((input.lines().last().unwrap().len() as i32 - 1) / 2);

    input.lines().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, c)| {
            match c {
                '#' => set.insert(((x as i32) - xdim, (y as i32) - ydim, 0, 0)),
                '.' => false,
                _ => unreachable!(),
            };
        })
    });
    set.clone()
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut cur = input.clone();
    let mut pre = input.clone();

    for i in 1..=CYCLES {
        std::mem::swap(&mut pre, &mut cur);
        for (x, y, z) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i) {
            let adjacent = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1)
                .into_iter()
                .filter(|(i, j, k)| pre.contains(&(*i, *j, *k, 0)))
                .count();
            match (pre.contains(&(x, y, z, 0)), adjacent) {
                (true, 3..=4) => cur.insert((x, y, z, 0)),
                (false, 3) => cur.insert((x, y, z, 0)),
                _ => cur.remove(&(x, y, z, 0))
            };
        }
    }
    cur.len()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let mut cur = input.clone();
    let mut pre = input.clone();

    for i in 1..=CYCLES {
        std::mem::swap(&mut pre, &mut cur);
        for (x, y, z, w) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, -i..=i) {
            let adjacent = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, w - 1..=w + 1)
                .into_iter()
                .filter(|(i, j, k, l)| pre.contains(&(*i, *j, *k, *l)))
                .count();
            match (pre.contains(&(x, y, z, w)), adjacent) {
                (true, 3..=4) => cur.insert((x, y, z, w)),
                (false, 3) => cur.insert((x, y, z, w)),
                _ => cur.remove(&(x, y, z, w)),
            };
        }
    }
    cur.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = ".#.
..#
###";

        assert_eq!(day17_part1(&day17_generator(sample)), 112);
    }

    #[test]
    pub fn test2() {
        let sample = ".#.
..#
###";

        assert_eq!(day17_part2(&day17_generator(sample)), 848);
    }
}
