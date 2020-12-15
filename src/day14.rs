use std::collections::{HashMap, HashSet};
use std::mem::swap;

pub struct MemModifier {
    m: String,
    mem: Vec<(u64, u64)>,
}

#[aoc_generator(day14)]
pub fn day14_generator(input: &str) -> Vec<MemModifier> {
    let mut writes: Vec<(u64, u64)> = Vec::new();
    let mut mask = "".to_string();
    let mut target_vec: Vec<MemModifier> = Vec::new();

    input.lines().into_iter().for_each(|line| {
        let parts: Vec<&str> = line
            .split('=')
            .into_iter()
            .map(|part| part.trim())
            .collect();
        if parts[0] == "mask" {
            target_vec.push(MemModifier {
                m: mask.clone(),
                mem: writes.clone(),
            });
            mask = parts[1].to_string();
            writes = Vec::new();
        } else {
            let (addr, val) = decode_mem_instruction(parts[0], parts[1]);
            writes.push((addr, val));
        }
    });
    target_vec.push(MemModifier {
        m: mask.clone(),
        mem: writes.clone(),
    });
    target_vec
}

pub fn decode_mem_instruction(inst: &str, val: &str) -> (u64, u64) {
    (
        inst[4..inst.len() - 1].parse().unwrap(),
        val.parse().unwrap(),
    )
}

pub fn decode_mask(val: String) -> (u64, u64) {
    let mut ones: u64 = 0;
    let mut zeroes: u64 = 0;
    val.char_indices().for_each(|(idx, c)| match c {
        '0' => zeroes |= 1 << (35 - idx),
        '1' => ones |= 1 << (35 - idx),
        _ => {}
    });
    zeroes = !zeroes;
    (zeroes, ones)
}

pub fn decode_mask_part2(val: String) -> (u64, Vec<u64>) {
    let mut ones: u64 = 0;
    let mut floating: Vec<u64> = Vec::new();
    val.char_indices().for_each(|(idx, c)| match c {
        '1' => ones |= 1 << (35 - idx),
        'X' => floating.push(35 - idx as u64),
        _ => {}
    });
    (ones, floating)
}

pub fn apply_mask(zeroes: u64, ones: u64, val: u64) -> u64 {
    (val | ones) & zeroes
}

#[aoc(day14, part1)]
pub fn day14_part1(input: &[MemModifier]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    input.iter().for_each(|mm| {
        mm.mem.iter().for_each(|(addr, data)| {
            let (z, o) = decode_mask(mm.m.clone());
            memory.insert(*addr, apply_mask(z, o, *data));
        })
    });
    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn day14_part2(input: &[MemModifier]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    input.iter().for_each(|mm| {
        mm.mem.iter().for_each(|(base, data)| {
            let (ones, floating) = decode_mask_part2(mm.m.clone());
            let addresses = generate_addresses_from_mask(*base, ones, &floating);
            assert_eq!(
                (2 as usize).pow(floating.len() as u32) as usize,
                addresses.len()
            );
            assert_eq!(floating.len(), mm.m.chars().filter(|&c| c == 'X').count());
            for a in addresses {
                memory.insert(a, *data);
            }
        });
    });
    memory.values().sum()
}

pub fn generate_addresses_from_mask(base: u64, ones: u64, modifies: &Vec<u64>) -> HashSet<u64> {
    let mut addr: HashSet<u64> = HashSet::new();
    let mut addr2: HashSet<u64> = HashSet::new();
    let base = base | ones;

    addr.insert(base);

    for i in modifies {
        swap(&mut addr, &mut addr2);
        for &a in &addr2 {
            addr.insert(a | (1 << i));
            addr.insert(a & !(1 << i));
        }
    }
    addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test0() {
        assert_eq!(
            apply_mask(
                0b00000000_00000000_00000000_00000000_00011000_00000000_00000000_00000000,
                0xFFFF_FFFF_FFFF_FFFF,
                0
            ),
            0b00000000_00000000_00000000_00000000_00011000_00000000_00000000_00000000
        );

        let (z, o) = decode_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!(
            apply_mask(z, o, 0b000000000000000000000000000000001011),
            0b000000000000000000000000000001001001
        );

        let (z, o) = decode_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!(
            apply_mask(z, o, 0b000000000000000000000000000001100101),
            0b000000000000000000000000000001100101
        );

        let (z, o) = decode_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!(
            apply_mask(z, o, 0b000000000000000000000000000000000000),
            0b000000000000000000000000000001000000
        );
    }

    #[test]
    pub fn test1() {
        let sample = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(day14_part1(&day14_generator(sample)), 165)
    }

    #[test]
    pub fn test2() {
        let sample = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(day14_part2(&day14_generator(sample)), 208)
    }
}
