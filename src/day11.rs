use itertools::Itertools;
use std::fmt;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum SeatStatus {
    Floor = 0,
    Occupied = 1,
    Empty = 2,
    DontCare = 255
}
impl fmt::Display for SeatStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SeatStatus::Floor => write!(f, "."),
            SeatStatus::Occupied => write!(f, "#"),
            SeatStatus::Empty => write!(f, "L"),
            _ => write!(f, "INVALID")
        }
    }
}

#[aoc_generator(day11)]
pub fn day11_generator(input: &str) -> Vec<Vec<SeatStatus>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| match c {
                '.' => SeatStatus::Floor,
                'L' => SeatStatus::Empty,
                '#' => SeatStatus::Occupied,
                _ => SeatStatus::DontCare
                 }).collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn day11_part1(input: &Vec<Vec<SeatStatus>>) -> usize {
    //print_state(input);
    iterate_placement(input, 1)
}

pub fn iterate_placement(input: &Vec<Vec<SeatStatus>>, iter_count: usize) -> usize {
    let mut new_seat_status = input.clone();

    for (x_coord, x) in input.iter().enumerate() {
        for (y_coord, y) in x.iter().enumerate() {
            //let it = (-1..=1).cartesian_product(-1..=1).into_iter().for_each(|x| println!("x: {}, y: {}", x.0, x.1));
            let adjacent: Vec<SeatStatus> = (-1..=1).cartesian_product(-1..=1).into_iter().map(|mv| {
                let i = mv.0;
                let j = mv.1;
                if i == 0 && j == 0 {
                    SeatStatus::DontCare
                } else if (x_coord as i32 + i < 0) || (x_coord as i32 + i >= input.len() as i32) || (y_coord as i32 + j  < 0) ||(y_coord as i32 + j >= x.len() as i32) {
                    SeatStatus::Empty
                } else {
                    let xn: usize = ((x_coord as i32) + i) as usize;
                    let yn: usize = ((y_coord as i32) + j) as usize;
                    //println!("x: {} y:{} xn: {} yn:{}", x_coord, y_coord, xn, yn);
                    input[xn][yn].clone()
                }
            }).collect();
            let near_occupied = adjacent.iter().fold(0, |acc, x| acc + if x == &SeatStatus::Occupied { 1 } else { 0 });
            if y == &SeatStatus::Empty && near_occupied == 0 {
                new_seat_status[x_coord][y_coord] = SeatStatus::Occupied;
            }
            if y == &SeatStatus::Occupied && near_occupied >= 4 {
                new_seat_status[x_coord][y_coord] = SeatStatus::Empty;
            }
        }
    }
    //print_state(&new_seat_status);

    if input
        .iter()
        .flatten()
        .zip(
            new_seat_status
                .iter()
                .flatten())
        .filter(|&(a, b)| a != b)
        .count() == 0
    {
        input.iter().flatten().fold(0, |acc, x| acc + if x == &SeatStatus::Occupied { 1 } else { 0 })
    } else {
        iterate_placement(&new_seat_status, iter_count + 1)
    }
}

pub fn print_state(input: &Vec<Vec<SeatStatus>>) {
    for x in input.iter() {
        for y in x.iter() {
            print!("{}", y)
        }
        println!()
    }
    println!()
}

#[aoc(day11, part2)]
pub fn day11_part2(input: &Vec<Vec<SeatStatus>>) -> usize {
    print_state(input);
    iterate_placement_part2(input, 1)
}

pub fn iterate_placement_part2(input: &Vec<Vec<SeatStatus>>, iter_count: usize) -> usize {
    let mut new_seat_status = input.clone();

    for (x_coord, x) in input.iter().enumerate() {
        for (y_coord, y) in x.iter().enumerate() {
            //let it = (-1..=1).cartesian_product(-1..=1).into_iter().for_each(|x| println!("x: {}, y: {}", x.0, x.1));
            let adjacent: Vec<SeatStatus> = (-1..=1).cartesian_product(-1..=1).into_iter().map(|mv| {
                let i = mv.0;
                let j = mv.1;
                if i == 0 && j == 0 {
                    SeatStatus::DontCare
                } else if (x_coord as i32 + i < 0) || (x_coord as i32 + i >= input.len() as i32) || (y_coord as i32 + j  < 0) ||(y_coord as i32 + j >= x.len() as i32) {
                    SeatStatus::Empty
                } else {
                    get_seat_in_direction(input, x_coord, y_coord, i, j)
                }
            }).collect();
            let near_occupied = adjacent.iter().fold(0, |acc, x| acc + if x == &SeatStatus::Occupied { 1 } else { 0 });
            if y == &SeatStatus::Empty && near_occupied == 0 {
                new_seat_status[x_coord][y_coord] = SeatStatus::Occupied;
            }
            if y == &SeatStatus::Occupied && near_occupied >= 5 {
                new_seat_status[x_coord][y_coord] = SeatStatus::Empty;
            }
        }
    }
    print_state(&new_seat_status);

    if input
        .iter()
        .flatten()
        .zip(
            new_seat_status
                .iter()
                .flatten())
        .filter(|&(a, b)| a != b)
        .count() == 0
    {
        input.iter().flatten().fold(0, |acc, x| acc + if x == &SeatStatus::Occupied { 1 } else { 0 })
    } else {
        iterate_placement_part2(&new_seat_status, iter_count + 1)
    }
}

pub fn get_seat_in_direction(seats: &Vec<Vec<SeatStatus>>, x: usize, y: usize, dx: i32, dy: i32) -> SeatStatus {
    let mut px = x as i32;
    let mut py = y as i32;
    loop {
        px += dx;
        py += dy;
        if px < 0 || py < 0 || px as usize >= seats.len() || py as usize >= seats[0].len() {
            return SeatStatus::Empty
        } else {
            if seats[px as usize][py as usize] == SeatStatus::Floor {
                return get_seat_in_direction(seats, px as usize, py as usize, dx, dy)
            } else {
                return seats[px as usize][py as usize]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let generated = day11_generator(sample);

        assert_eq!(day11_part1(&generated), 37)
    }

    #[test]
    pub fn test2() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let generated = day11_generator(sample);

        assert_eq!(day11_part2(&generated), 26)
    }
}