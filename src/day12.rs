#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NavInstruction {
    action: char,
    val: i32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ShipStatus {
    e: i32,
    n: i32,
    d: i32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ShipStatusWP {
    e: i32,
    n: i32,
    d: i32,
    wp_e: i32,
    wp_n: i32,
}

trait Manhattan {
    fn manhattan(&self) -> i32;
}

impl Manhattan for ShipStatus {
    fn manhattan(&self) -> i32 {
        self.e.abs() + self.n.abs()
    }
}

impl Manhattan for ShipStatusWP {
    fn manhattan(&self) -> i32 {
        self.e.abs() + self.n.abs()
    }
}

#[aoc_generator(day12)]
pub fn day12_generator(input: &str) -> Vec<NavInstruction> {
    input
        .lines()
        .map(|line| NavInstruction {
            action: line.chars().next().unwrap(),
            val: line[1..].parse().unwrap(),
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn day12_part1(input: &Vec<NavInstruction>) -> i32 {
    let endpoint = input
        .iter()
        .scan(ShipStatus { e: 0, n: 0, d: 0 }, |status, &i| {
            *status = move_ship(status.clone(), &i);
            Some(*status)
        })
        .last()
        .unwrap();
    endpoint.manhattan()
}

#[aoc(day12, part2)]
pub fn day12_part2(input: &Vec<NavInstruction>) -> i32 {
    let endpoint = input
        .iter()
        .scan(
            ShipStatusWP {
                e: 0,
                n: 0,
                wp_e: 10,
                wp_n: 1,
                d: 0,
            },
            |status, &i| {
                *status = move_ship_wp(status.clone(), &i);
                Some(*status)
            },
        )
        .last()
        .unwrap();
    endpoint.manhattan()
}

pub fn move_ship(s: ShipStatus, i: &NavInstruction) -> ShipStatus {
    match i.action {
        'E' => move_by_direction(s, 0, i.val),
        'S' => move_by_direction(s, 90, i.val),
        'W' => move_by_direction(s, 180, i.val),
        'N' => move_by_direction(s, 270, i.val),
        'L' => ShipStatus {
            e: s.e,
            n: s.n,
            d: (s.d - i.val + 360) % 360,
        },
        'R' => ShipStatus {
            e: s.e,
            n: s.n,
            d: (s.d + i.val + 360) % 360,
        },
        'F' => move_by_direction(s, s.d, i.val),
        _ => ShipStatus { e: 0, n: 0, d: 0 },
    }
}

pub fn move_ship_wp(s: ShipStatusWP, i: &NavInstruction) -> ShipStatusWP {
    match i.action {
        'E' => move_wp_by_direction(s, 0, i.val),
        'S' => move_wp_by_direction(s, 90, i.val),
        'W' => move_wp_by_direction(s, 180, i.val),
        'N' => move_wp_by_direction(s, 270, i.val),
        'L' => rotate_wp(s, i.val),
        'R' => rotate_wp(s, -i.val),
        'F' => move_ship_to_wp(s, i.val),
        _ => ShipStatusWP {
            e: 0,
            n: 0,
            wp_e: 0,
            wp_n: 0,
            d: 0,
        },
    }
}

pub fn move_ship_to_wp(s: ShipStatusWP, val: i32) -> ShipStatusWP {
    ShipStatusWP {
        e: s.e + val * s.wp_e,
        n: s.n + val * s.wp_n,
        wp_e: s.wp_e,
        wp_n: s.wp_n,
        d: s.d,
    }
}

pub fn rotate_wp(s: ShipStatusWP, val: i32) -> ShipStatusWP {
    let val = ((val + 360 % 360) as f64).to_radians();

    let sin = val.sin();
    let cos = val.cos();

    let nwp_e = f64::round(cos * s.wp_e as f64 - sin * s.wp_n as f64) as i32;
    let nwp_n = f64::round(sin * s.wp_e as f64 + cos * s.wp_n as f64) as i32;

    ShipStatusWP {
        e: s.e,
        n: s.n,
        wp_e: nwp_e,
        wp_n: nwp_n,
        d: s.d,
    }
}

pub fn move_wp_by_direction(s: ShipStatusWP, d: i32, val: i32) -> ShipStatusWP {
    match d {
        // EAST
        0 => ShipStatusWP {
            e: s.e,
            n: s.n,
            wp_e: s.wp_e + val,
            wp_n: s.wp_n,
            d: s.d,
        },
        // SOUTH
        90 => ShipStatusWP {
            e: s.e,
            n: s.n,
            wp_e: s.wp_e,
            wp_n: s.wp_n - val,
            d: s.d,
        },
        // EAST
        180 => ShipStatusWP {
            e: s.e,
            n: s.n,
            wp_e: s.wp_e - val,
            wp_n: s.wp_n,
            d: s.d,
        },
        // NORTH
        270 => ShipStatusWP {
            e: s.e,
            n: s.n,
            wp_e: s.wp_e,
            wp_n: s.wp_n + val,
            d: s.d,
        },
        _ => ShipStatusWP {
            e: 0,
            n: 0,
            wp_e: 0,
            wp_n: 0,
            d: 0,
        },
    }
}

pub fn move_by_direction(s: ShipStatus, d: i32, val: i32) -> ShipStatus {
    match d {
        // EAST
        0 => ShipStatus {
            e: s.e + val,
            n: s.n,
            d: s.d,
        },
        // SOUTH
        90 => ShipStatus {
            e: s.e,
            n: s.n - val,
            d: s.d,
        },
        // WEST
        180 => ShipStatus {
            e: s.e - val,
            n: s.n,
            d: s.d,
        },
        // NORTH
        270 => ShipStatus {
            e: s.e,
            n: s.n + val,
            d: s.d,
        },
        _ => ShipStatus { e: 0, n: 0, d: 0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = "F10
N3
F7
R90
F11";
        let gen = day12_generator(sample);
        assert_eq!(day12_part1(&gen), 25)
    }

    #[test]
    pub fn test2() {
        let sample = "F10
N3
F7
R90
F11";
        let gen = day12_generator(sample);
        let state = ShipStatusWP {
            e: 0,
            n: 0,
            wp_e: 10,
            wp_n: 1,
            d: 0,
        };

        let state = move_ship_wp(state, &gen[0]);
        assert_eq!(state.e, 100);
        assert_eq!(state.n, 10);
        assert_eq!(state.wp_e, 10);
        assert_eq!(state.wp_n, 1);

        let state = move_ship_wp(state, &gen[1]);
        assert_eq!(state.e, 100);
        assert_eq!(state.n, 10);
        assert_eq!(state.wp_e, 10);
        assert_eq!(state.wp_n, 4);

        let state = move_ship_wp(state, &gen[2]);
        assert_eq!(state.e, 170);
        assert_eq!(state.n, 38);
        assert_eq!(state.wp_e, 10);
        assert_eq!(state.wp_n, 4);

        let state = move_ship_wp(state, &gen[3]);
        assert_eq!(state.e, 170);
        assert_eq!(state.n, 38);
        assert_eq!(state.wp_e, 4);
        assert_eq!(state.wp_n, -10);

        let state = move_ship_wp(state, &gen[4]);
        assert_eq!(state.e, 214);
        assert_eq!(state.n, -72);
        assert_eq!(state.wp_e, 4);
        assert_eq!(state.wp_n, -10);
    }
}
