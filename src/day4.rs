use regex::Regex;
use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn day4_generator(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.to_string()).collect()
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &Vec<String>) -> usize {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .map(|x| has_required_fields(&x, &required_fields) as usize)
        .sum()
}

#[aoc(day4, part2)]
pub fn day4_part2(input: &Vec<String>) -> usize {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|x| has_required_fields(&x, &required_fields))
        .map(|x| has_valid_fields(&x) as usize)
        .sum()
}

pub fn has_required_fields(passport: &String, required_fields: &Vec<&str>) -> bool {
    required_fields.iter().all(|&req| passport.contains(req))
}

fn has_valid_fields(passport: &str) -> bool {
    let fields_separator = Regex::new(r"\s\n|\s|\n").unwrap();

    // Drop the final nextline to avoid
    let fields: Vec<&str> = fields_separator
        .split(passport)
        .filter(|field| !field.is_empty())
        .collect();

    for f in fields.iter() {
        println!("Fields: {}", f)
    }

    fields.iter().all(|field| is_valid_field(field))
}

pub fn is_valid_field(field: &str) -> bool {
    let splitted: Vec<&str> = field.split(':').collect();
    let (k, v) = (splitted[0], splitted[1]);

    match k {
        "byr" => is_in_range(v, 1920..=2002),
        "iyr" => is_in_range(v, 2010..=2020),
        "eyr" => is_in_range(v, 2020..=2030),
        "hgt" => is_in_range_suffix(v, 150..=193, "cm") || is_in_range_suffix(v, 59..=76, "in"),
        "hcl" => v
            .strip_prefix("#")
            .map_or(false, |color| color.chars().all(|c| c.is_ascii_hexdigit())),
        "ecl" => matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => v.len() == 9 && v.chars().all(|d| d.is_ascii_digit()),
        "cid" => true,
        _ => false,
    }
}

fn is_in_range(value: &str, range: RangeInclusive<usize>) -> bool {
    value
        .parse()
        .map_or(false, |num: usize| range.contains(&num))
}

fn is_in_range_suffix(value: &str, range: RangeInclusive<usize>, suffix: &str) -> bool {
    value
        .strip_suffix(suffix)
        .map_or(false, |x| is_in_range(x, range))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let sample = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let generated = day4_generator(sample);
        assert_eq!(day4_part1(&generated), 2)
    }

    #[test]
    pub fn test2() {
        assert!(is_valid_field("byr:2002"));
        assert!(!is_valid_field("byr:2003"));

        assert!(is_valid_field("hgt:60in"));
        assert!(is_valid_field("hgt:190cm"));
        assert!(!is_valid_field("hgt:190in"));
        assert!(!is_valid_field("hgt:190"));

        assert!(is_valid_field("hcl:#123abc"));
        assert!(!is_valid_field("hcl:#123abz"));
        assert!(!is_valid_field("hcl:123abc"));

        assert!(is_valid_field("ecl:brn"));
        assert!(!is_valid_field("ecl:wat"));

        assert!(is_valid_field("pid:000000001"));
        assert!(!is_valid_field("pid:0123456789"));
        assert!(!is_valid_field("pid:0000a0001"));
    }

    #[test]
    pub fn test2_passports() {
        let sample = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let generated = day4_generator(sample);
        assert_eq!(day4_part2(&generated), 4)
    }
}
