use regex::Regex;

#[derive(Debug)]
pub struct Password {
    lower: usize,
    upper: usize,
    ch: char,
    password: String
}

impl Password {
    fn is_valid_part_1(&self) -> bool {
        (self.lower..self.upper+1).contains(&self.password.matches(self.ch).count())
    }

    fn is_valid_part_2(&self) -> bool {
        self.password
            .char_indices()
            .filter(|(index, _)| self.lower == index +1 || self.upper == index + 1)
            .filter(|(_, c)| &self.ch == c)
            .count() == 1
    }
}

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Vec<Password> {
    let re = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();

    input
        .lines()
        .map(|l| {
            let captures = re.captures(l.trim()).expect("No match found");
            Password{
                lower: captures[1].parse().unwrap(),
                upper: captures[2].parse().unwrap(),
                ch: captures[3].parse().unwrap(),
                password: captures[4].to_string()
                }
        }).collect()
}

#[aoc(day2, part1)]
pub fn day2_part1(input: &Vec<Password>) -> usize {
    input.iter().filter(|x| x.is_valid_part_1()).count()
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &Vec<Password>) -> usize {
    input.iter().filter(|x| x.is_valid_part_2()).count()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test1() {
            let sample = "1-3 a: abcde,
            1-3 b: cdefg
            2-9 c: cc
            2-9 c: cccccc
            2-9 c: ccccccccc
            2-9 c: cccccccccc";
        let generated = day2_generator(sample);
        //for x in generated.iter() {
        //    println!("To {} From {} ch {} Password {}", x.lower, x.upper, x.ch, x.password);
        //    println!("Actual: {} Valid {}", x.password.matches(x.ch).count(), x.is_valid_part_1())
        //}
        assert_eq!(day2_part1(&generated), 4)
    }

    #[test]
    pub fn test2() {
        let sample = "1-3 a: abcde,
            1-3 b: cdefg
            2-9 c: cc
            2-9 c: cccccc
            2-9 c: ccccccccc
            2-9 c: cccccccccc";
        let generated = day2_generator(sample);
        //for x in generated.iter() {
        //    println!("To {} From {} ch {} Password {}", x.lower, x.upper, x.ch, x.password);
        //    println!("Actual: {} Valid {}", x.password.matches(x.ch).count(), x.is_valid_part_1())
        //}
        assert_eq!(day2_part1(&generated), 4)
    }
}