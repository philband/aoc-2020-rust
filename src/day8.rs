use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum OperationCode {
    Nop,
    Acc,
    Jmp,
    Invalid,
}

impl fmt::Display for OperationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OperationCode::Nop => write!(f, "nop"),
            OperationCode::Acc => write!(f, "acc"),
            OperationCode::Jmp => write!(f, "jmp"),
            _ => write!(f, "INVALID"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Operation {
    opcode: OperationCode,
    val: i32,
}

#[aoc_generator(day8)]
pub fn day8_generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<String> = line.split(' ').map(|x| x.to_string()).collect();
            Operation {
                opcode: match parts[0].as_str() {
                    "nop" => OperationCode::Nop,
                    "acc" => OperationCode::Acc,
                    "jmp" => OperationCode::Jmp,
                    _ => OperationCode::Invalid,
                },
                val: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn day8_part1(input: &Vec<Operation>) -> i32 {
    part1_recurse(input, 0, 0, HashSet::new())
}

pub fn part1_recurse(
    input: &Vec<Operation>,
    index: i32,
    accumulator: i32,
    visited: HashSet<i32>,
) -> i32 {
    if visited.contains(&index) || index as usize >= input.len() {
        accumulator
    } else {
        let current = &input[index as usize];
        let mut next_visited = visited.clone();
        next_visited.insert(index);

        part1_recurse(
            input,
            match current.opcode {
                OperationCode::Jmp => index + current.val,
                _ => index + 1,
            },
            match current.opcode {
                OperationCode::Acc => accumulator + current.val,
                _ => accumulator,
            },
            next_visited,
        )
    }
}

#[aoc(day8, part2)]
pub fn day8_part2(input: &Vec<Operation>) -> i32 {
    for (k, &v) in input.iter().enumerate() {
        if v.opcode == OperationCode::Acc {
            continue;
        }
        let mut test_vec = input.clone();
        test_vec[k] = Operation {
            opcode: match v.opcode {
                OperationCode::Jmp => OperationCode::Nop,
                OperationCode::Nop => OperationCode::Jmp,
                _ => v.opcode,
            },
            val: v.val,
        };
        let ret_val = part2_recurse(&test_vec, 0, 0, HashSet::new());
        if ret_val > 0 {
            return ret_val;
        }
    }
    0
}

pub fn part2_recurse(
    input: &Vec<Operation>,
    index: i32,
    accumulator: i32,
    visited: HashSet<i32>,
) -> i32 {
    if index as usize >= input.len() {
        accumulator
    } else if visited.contains(&index) {
        0
    } else {
        let current = &input[index as usize];
        let mut next_visited = visited.clone();
        next_visited.insert(index);

        part2_recurse(
            input,
            match current.opcode {
                OperationCode::Jmp => index + current.val,
                _ => index + 1,
            },
            match current.opcode {
                OperationCode::Acc => accumulator + current.val,
                _ => accumulator,
            },
            next_visited,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let generated = day8_generator(&sample);
        assert_eq!(day8_part1(&generated), 5)
    }

    #[test]
    pub fn test2() {
        let sample = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let generated = day8_generator(&sample);

        assert_eq!(day8_part2(&generated), 8)
    }
}
