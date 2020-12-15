use itertools::Itertools;

#[derive(Debug)]
pub struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    pub fn seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

#[aoc_generator(day5)]
pub fn day5_generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|l| {
            let mut low = 0;
            let mut mutator = 64;

            for c in l[..7].chars() {
                if c == 'B' {
                    low = low + mutator;
                }
                mutator /= 2;
            }
            let row = low;

            mutator = 4;
            low = 0;
            for c in l[7..].chars() {
                if c == 'R' {
                    low = low + mutator;
                }
                mutator /= 2;
            }
            let col = low;
            Seat { row, col }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn day5_part1(input: &Vec<Seat>) -> u32 {
    input.iter().map(|x| x.seat_id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn day5_part2(input: &Vec<Seat>) -> u32 {
    let seats = input.iter().map(|x| x.seat_id()).collect::<Vec<_>>();
    let max: u32 = *seats.iter().max().unwrap();
    let min: u32 = *seats.iter().min().unwrap();

    (min..max)
        .tuple_windows()
        .filter_map(|(b, s, a)| {
            match (seats.contains(&b), seats.contains(&s), seats.contains(&a)) {
                (true, false, true) => Some(s),
                _ => None,
            }
        })
        .next()
        .unwrap()
}

#[aoc(day5, part2, sum)]
pub fn day5_part2_sum(input: &Vec<Seat>) -> u32 {
    let seats = input.iter().map(|x| x.seat_id()).collect::<Vec<_>>();
    let max = seats.iter().max().unwrap();
    let min = seats.iter().min().unwrap();
    let sum = seats.iter().sum::<u32>();

    (max * (max + 1) / 2) - sum - ((min - 1) * min / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

        let generated = day5_generator(sample);
        assert_eq!(generated.len(), 3);
        assert_eq!(generated[0].row, 70);
        assert_eq!(generated[1].row, 14);
        assert_eq!(generated[2].row, 102);
        assert_eq!(generated[0].col, 7);
        assert_eq!(generated[1].col, 7);
        assert_eq!(generated[2].col, 4);
        assert_eq!(generated[0].seat_id(), 567);
        assert_eq!(generated[1].seat_id(), 119);
        assert_eq!(generated[2].seat_id(), 820);

        assert_eq!(day5_part1(&generated), 820)
    }
}
