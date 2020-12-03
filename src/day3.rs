
#[aoc_generator(day3)]
pub fn day3_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.char_indices().map(|(_, x)| if x == '#' {true} else {false}).collect()
        }).collect()
}

#[aoc(day3, part1)]
pub fn day3_part1(input: &Vec<Vec<bool>>) -> usize {
    find_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn day3_part2(input: &Vec<Vec<bool>>) -> usize {
    find_trees(input, 1, 1)
    * find_trees(input, 3, 1)
    * find_trees(input, 5, 1)
    * find_trees(input, 7, 1)
    * find_trees(input, 1, 2)
}

pub fn find_trees(input: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
    let mut index = 0;
    input
        .iter()
        .skip(1)
        .enumerate()
        .filter_map(|(i, l)| {
            if (i+1) % down == 0 {
                index += right;
                //println!("target {} converted {} data {}", index, index % l.len(), l[index % l.len()]);
                Some(l[index % l.len()])
            } else {
                None
            }
        })
        .map(|b| if b {1} else {0})
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test1() {
        let sample =
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let generated = day3_generator(sample);
        //for x in generated.iter() {
        //    println!("To {} From {} ch {} Password {}", x.lower, x.upper, x.ch, x.password);
        //    println!("Actual: {} Valid {}", x.password.matches(x.ch).count(), x.is_valid_part_1())
        //}
        assert_eq!(day3_part1(&generated), 7)
    }

    #[test]
    pub fn test2() {
        let sample =
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let generated = day3_generator(sample);
        //for x in generated.iter() {
        //    println!("To {} From {} ch {} Password {}", x.lower, x.upper, x.ch, x.password);
        //    println!("Actual: {} Valid {}", x.password.matches(x.ch).count(), x.is_valid_part_1())
        //}
        assert_eq!(find_trees(&generated, 1, 1), 2);
        assert_eq!(find_trees(&generated, 3, 1), 7);
        assert_eq!(find_trees(&generated, 5, 1), 3);
        assert_eq!(find_trees(&generated, 7, 1), 4);
        assert_eq!(find_trees(&generated, 1, 2), 2);
        assert_eq!(day3_part2(&generated), 336)

    }
}