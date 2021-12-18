use std::str::FromStr;

#[derive(Debug)]
pub enum Snailfish {
    Number(i32),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    pub fn parse(s: &str) -> (Self, &str) {
        match s.strip_prefix('[') {
            Some(s) => {
                let (left, s) = Self::parse(s);
                let s = s.strip_prefix(',').unwrap();
                let (right, s) = Self::parse(s);
                let s = s.strip_prefix(']').unwrap();
                let pair = Snailfish::Pair(Box::new(left), Box::new(right));
                (pair, s)
            }
            None => {
                let (digits, s) = match s.find(|c: char| !c.is_ascii_digit()) {
                    Some(pos) => s.split_at(pos),
                    None => (s, ""),
                };
                let number = Snailfish::Number(digits.parse().unwrap());
                (number, s)
            }
        }
    }
}

impl FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, s) = Self::parse(s);
        assert!(s.is_empty());
        Ok(result)
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Snailfish> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Snailfish]) -> i32 {
    dbg!(&input);
    todo!()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Snailfish]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"".trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
