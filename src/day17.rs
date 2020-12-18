use itertools::Itertools;
use std::collections::HashMap;
use std::mem::swap;

#[aoc_generator(day17)]
pub fn day17_generator(input: &str) -> HashMap<(i32, i32, i32, i32), bool> {
    let mut map: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
    let rs: i32 = 11;
    for x in -rs..=rs {
        for y in -rs..=rs {
            for z in -rs..=rs {
                for w in -rs..=rs {
                    map.insert((x, y, z, w), false);
                }
            }
        }
    }
    input.lines().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, c)| {
            let state: bool = match c {
                '#' => true,
                _ => false,
            };
            //println!("x: {}, y: {} => {}", (x as i32) - ((input.lines().enumerate().last().unwrap().0 as i32)/2), (y as i32) - ((line.len() as i32-1)/2), state);
            map.insert(
                (
                    (x as i32) - ((input.lines().enumerate().last().unwrap().0 as i32) / 2),
                    (y as i32) - ((line.len() as i32 - 1) / 2),
                    0,
                    0,
                ),
                state,
            );
        })
    });
    map.clone()
}

#[aoc(day17, part1)]
pub fn day17_part1(input: &HashMap<(i32, i32, i32, i32), bool>) -> i32 {
    let mut input1 = input.clone();
    let mut input2 = input.clone();
    let rs: i32 = 10;
    //println!("Before:");
    //print_state(&input2, 1, 1, 0);
    //println!();
    //println!();

    for i in 0..6 {
        for x in -rs..=rs {
            for y in -rs..=rs {
                for z in -rs..=rs {
                    let adjacent: i32 = (-1..=1)
                        .cartesian_product(-1..=1)
                        .cartesian_product(-1..=1)
                        .into_iter()
                        .map(|((i, j), k)| {
                            if i == 0 && j == 0 && k == 0 {
                                0
                            } else {
                                if input2.contains_key(&(x - i, y - j, z - k, 0)) {
                                    if *input2.get(&(x - i, y - j, z - k, 0)).unwrap() {
                                        1
                                    } else {
                                        0
                                    }
                                } else {
                                    0
                                }
                            }
                        })
                        .sum();
                    input1.insert(
                        (x, y, z, 0),
                        match input2.get(&(x, y, z, 0)).unwrap() {
                            true => {
                                if adjacent == 2 || adjacent == 3 {
                                    true
                                } else {
                                    false
                                }
                            }
                            false => {
                                if adjacent == 3 {
                                    true
                                } else {
                                    false
                                }
                            }
                        },
                    );
                }
            }
        }

        swap(&mut input1, &mut input2);
        //print_state(&input2, i+1, i+1, i+1);
    }

    (-rs..=rs)
        .cartesian_product(-rs..=rs)
        .cartesian_product(-rs..=rs)
        .into_iter()
        .map(|((x, y), z)| {
            if *input2.get(&(x, y, z, 0)).unwrap() {
                1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day17, part2)]
pub fn day17_part2(input: &HashMap<(i32, i32, i32, i32), bool>) -> i32 {
    let mut input1 = input.clone();
    let mut input2 = input.clone();
    let rs: i32 = 10;

    for i in 0..6 {
        for x in -rs..=rs {
            for y in -rs..=rs {
                for z in -rs..=rs {
                    for w in -rs..=rs {
                        let adjacent: i32 = (-1..=1)
                            .cartesian_product(-1..=1)
                            .cartesian_product(-1..=1)
                            .cartesian_product(-1..=1)
                            .into_iter()
                            .map(|(((i, j), k), l)| {
                                if i == 0 && j == 0 && k == 0 && l == 0 {
                                    0
                                } else {
                                    if input2.contains_key(&(x - i, y - j, z - k, w - l)) {
                                        if *input2.get(&(x - i, y - j, z - k, w - l)).unwrap() {
                                            1
                                        } else {
                                            0
                                        }
                                    } else {
                                        0
                                    }
                                }
                            })
                            .sum();
                        input1.insert(
                            (x, y, z, w),
                            match input2.get(&(x, y, z, w)).unwrap() {
                                true => {
                                    if adjacent == 2 || adjacent == 3 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                                false => {
                                    if adjacent == 3 {
                                        true
                                    } else {
                                        false
                                    }
                                }
                            },
                        );
                    }
                }
            }
        }

        swap(&mut input1, &mut input2);
        //print_state(&input2, i+1, i+1, i+1);
    }

    (-rs..=rs)
        .cartesian_product(-rs..=rs)
        .cartesian_product(-rs..=rs)
        .cartesian_product(-rs..=rs)
        .into_iter()
        .map(|(((x, y), z), w)| {
            if *input2.get(&(x, y, z, w)).unwrap() {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn print_state(input: &HashMap<(i32, i32, i32), bool>, xpm: i32, ypm: i32, zpm: i32) {
    for z in -zpm..=zpm {
        println!("z={}", z);
        for x in -xpm..=xpm {
            for y in -ypm..=ypm {
                if *input.get(&(x, y, z)).unwrap() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    println!();
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
