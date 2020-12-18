use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;
use std::mem::swap;

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
    let cycles = 6;
    let mut input1 = input.clone();
    let mut input2 = input.clone();

    for i in 1..=cycles {
        for (x, y, z) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i) {
            let adjacent = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1)
                .into_iter()
                .filter(|(i, j, k)| input2.contains(&(*i, *j, *k, 0)))
                .count();
            match (input2.contains(&(x, y, z, 0)), adjacent) {
                (true, 3..=4) => input1.insert((x, y, z, 0)),
                (false, 3) => input1.insert((x, y, z, 0)),
                _ => input1.remove(&(x, y, z, 0))
            };
        }
        swap(&mut input1, &mut input2);
    }

    iproduct!(
        -(cycles + 4)..=(cycles + 4),
        -(cycles + 4)..=(cycles + 4),
        -cycles..=cycles
    )
    .into_iter()
    .filter(|(x, y, z)| input2.contains(&(*x, *y, *z, 0)))
    .count()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &HashSet<(i32, i32, i32, i32)>) -> usize {
    let cycles = 6;
    let mut input1 = input.clone();
    let mut input2 = input.clone();

    for i in 1..=cycles {
        for (x, y, z, w) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, -i..=i) {
            let adjacent = iproduct!(x - 1..=x + 1, y - 1..=y + 1, z - 1..=z + 1, w - 1..=w + 1)
                .into_iter()
                .filter(|(i, j, k, l)| input2.contains(&(*i, *j, *k, *l)))
                .count();
            match (input2.contains(&(x, y, z, w)), adjacent) {
                (true, 3..=4) => input1.insert((x, y, z, w)),
                (false, 3) => input1.insert((x, y, z, w)),
                _ => input1.remove(&(x, y, z, w))
            };
        }
        swap(&mut input1, &mut input2);
    }

    iproduct!(
        -(cycles + 4)..=(cycles + 4),
        -(cycles + 4)..=(cycles + 4),
        -cycles..=cycles,
        -cycles..=cycles
    )
    .into_iter()
    .filter(|(x, y, z, w)| input2.contains(&(*x, *y, *z, *w)))
    .count()
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
