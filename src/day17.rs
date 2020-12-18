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
        for loc in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, 0..=0) {
            apply_rules(&pre, &mut cur, loc, count_neighbors3(loc, &pre));
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
        for loc in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, -i..=i) {
            apply_rules(&pre, &mut cur, loc, count_neighbors4(loc, &pre));
        }
    }
    cur.len()
}
fn count_neighbors3(loc: (i32, i32, i32, i32), pre: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let (x, y, z, w) = loc;
    iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1)
        .into_iter()
        .filter(|(i, j, k)| *&pre.contains(&(*i, *j, *k, w)))
        .count()
}

fn count_neighbors4(loc: (i32, i32, i32, i32), pre: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let (x, y, z, w) = loc;
    iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, w - 1..=w + 1)
        .into_iter()
        .filter(|(i, j, k, l)| *&pre.contains(&(*i, *j, *k, *l)))
        .count()
}

fn apply_rules(pre: &HashSet<(i32, i32, i32, i32)>, set: &mut HashSet<(i32, i32, i32, i32)>, loc: (i32, i32, i32, i32), cnt: usize) {
    match (pre.contains(&loc), cnt) {
        (true, 3..=4) => set.insert(loc),
        (false, 3) => set.insert(loc),
        _ => set.remove(&loc),
    };
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
