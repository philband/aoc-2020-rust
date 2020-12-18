use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;
use std::mem::swap;

const CYCLES: i32 = 6;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coord(i32, i32, i32, i32);

#[aoc_generator(day17)]
pub fn day17_generator(input: &str) -> HashSet<Coord> {
    let mut set: HashSet<Coord> = HashSet::new();
    let xdim = ((input.lines().enumerate().last().unwrap().0 as i32) / 2);
    let ydim = ((input.lines().last().unwrap().len() as i32 - 1) / 2);

    input.lines().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, c)| {
            match c {
                '#' => set.insert(Coord((x as i32) - xdim, (y as i32) - ydim, 0, 0)),
                '.' => false,
                _ => unreachable!(),
            };
        })
    });
    set.clone()
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &HashSet<Coord>) -> usize {
    let mut cur = input.clone();
    let mut pre = input.clone();

    for i in 1..=CYCLES {
        std::mem::swap(&mut pre, &mut cur);
        for loc in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, 0..=0) {
            let loc = Coord(loc.0, loc.1, loc.2, loc.3);
            apply_rules(&pre, &mut cur, loc, count_neighbors3(loc, &pre));
        }
    }
    cur.len()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &HashSet<Coord>) -> usize {
    let mut cur = input.clone();
    let mut pre = input.clone();

    for i in 1..=CYCLES {
        std::mem::swap(&mut pre, &mut cur);
        for loc in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, -i..=i) {
            let loc = Coord(loc.0, loc.1, loc.2, loc.3);
            apply_rules(&pre, &mut cur, loc, count_neighbors4(loc, &pre));
        }
    }
    cur.len()
}
fn count_neighbors3(loc: Coord, pre: &HashSet<Coord>) -> usize {
    iproduct!(
        loc.0 - 1..=loc.0 + 1,
        loc.1 - 1..=loc.1 + 1,
        loc.2 - 1..=loc.2 + 1
    )
    .into_iter()
    .filter(|(i, j, k)| *&pre.contains(&Coord(*i, *j, *k, loc.3)))
    .count()
}

fn count_neighbors4(loc: Coord, pre: &HashSet<Coord>) -> usize {
    iproduct!(
        loc.0 - 1..=loc.0 + 1,
        loc.1 - 1..=loc.1 + 1,
        loc.2 - 1..=loc.2 + 1,
        loc.3 - 1..=loc.3 + 1
    )
    .into_iter()
    .filter(|(i, j, k, l)| *&pre.contains(&Coord(*i, *j, *k, *l)))
    .count()
}

fn apply_rules(pre: &HashSet<Coord>, set: &mut HashSet<Coord>, loc: Coord, cnt: usize) {
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
