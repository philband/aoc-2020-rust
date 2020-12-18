use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashMap;
use std::mem::swap;

#[aoc_generator(day17)]
pub fn day17_generator(input: &str) -> HashMap<(i32, i32, i32, i32), bool> {
    let mut map: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    let cycles = 6;
    let xdim = ((input.lines().enumerate().last().unwrap().0 as i32) / 2);
    let ydim = ((input.lines().last().unwrap().len() as i32 - 1) / 2);
    let xrange = xdim + cycles;
    let yrange = ydim + cycles;
    let zwrange = cycles;

    for (x, y, z, w) in iproduct!(-xrange..=xrange, -yrange..=yrange, -zwrange..=zwrange, -zwrange..=zwrange) {
        map.insert((x, y, z, w), false);
    }
    input.lines().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, c)| {
            let state: bool = match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            };
            map.insert(((x as i32) - xdim, (y as i32) - ydim, 0, 0), state);
        })
    });
    map.clone()
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &HashMap<(i32, i32, i32, i32), bool>) -> i32 {
    let cycles = 6;
    let mut input1 = input.clone();
    let mut input2 = input.clone();

    for i in 1..=cycles {
        for (x, y, z) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i) {
            let adjacent: i32 = iproduct!(x-1..=x+1, y-1..=y+1, z-1..=z+1)
                .into_iter()
                .map(|(i, j, k)| match input2.get(&(i, j, k, 0)) {
                    Some(true) => 1,
                    _ => 0,
                })
                .sum();
            input1.insert(
                (x, y, z, 0),
                match (input2.get(&(x, y, z, 0)), adjacent) {
                    (Some(true), 3..=4) => true,
                    (Some(false), 3) => true,
                    _ => false,
                },
            );
        }
        swap(&mut input1, &mut input2);
    }

    iproduct!(
        -(cycles+4)..=(cycles+4),
        -(cycles+4)..=(cycles+4),
        -cycles..=cycles
    )
        .into_iter()
        .map(|(x, y, z)| match input2.get(&(x, y, z, 0)) {
            Some(true) => 1,
            _ => 0,
        })
        .sum()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &HashMap<(i32, i32, i32, i32), bool>) -> i32 {
    let cycles = 6;
    let mut input1 = input.clone();
    let mut input2 = input.clone();

    for i in 1..=cycles {
        for (x, y, z, w) in iproduct!(-(i + 3)..=(i + 3), -(i + 3)..=(i + 3), -i..=i, -i..=i) {
            let adjacent: i32 = iproduct!(x-1..=x+1, y-1..=y+1, z-1..=z+1, w-1..=w+1)
                .into_iter()
                .map(
                    |(i, j, k, l)| match input2.get(&(i, j, k, l)) {
                        Some(true) => 1,
                        _ => 0,
                    },
                )
                .sum();
            input1.insert(
                (x, y, z, w),
                match (input2.get(&(x, y, z, w)), adjacent) {
                    (Some(true), 3..=4) => true,
                    (Some(false), 3) => true,
                    _ => false,
                },
            );
        }
        swap(&mut input1, &mut input2);
    }

    iproduct!(
        -(cycles+4)..=(cycles+4),
        -(cycles+4)..=(cycles+4),
        -cycles..=cycles,
        -cycles..=cycles
    )
    .into_iter()
    .map(|(x, y, z, w)| match input2.get(&(x, y, z, w)) {
        Some(true) => 1,
        _ => 0,
    })
    .sum()
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
