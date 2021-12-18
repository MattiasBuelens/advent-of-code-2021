use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
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

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Snailfish::Number(x) => write!(f, "{}", x),
            Snailfish::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Snailfish> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

enum ExplodeResult {
    Failed,
    Success(Option<i32>, Option<i32>),
}

impl Snailfish {
    pub fn add(mut self, other: Self) -> Self {
        self = Snailfish::Pair(Box::new(self), Box::new(other));
        self.reduce();
        self
    }

    pub fn reduce(&mut self) {
        while self.try_explode() || self.try_split() {}
    }

    pub fn try_explode(&mut self) -> bool {
        matches!(self.explode_inner(0), ExplodeResult::Success(_, _))
    }

    fn explode_inner(&mut self, depth: usize) -> ExplodeResult {
        if let Snailfish::Pair(left, right) = self {
            if depth >= 4 {
                match (left.as_ref(), right.as_ref()) {
                    (&Snailfish::Number(left), &Snailfish::Number(right)) => {
                        // Explode this pair!
                        *self = Snailfish::Number(0);
                        return ExplodeResult::Success(Some(left), Some(right));
                    }
                    _ => {}
                }
            }
            // Carry numbers from exploded pair to adjacent pairs
            match left.explode_inner(depth + 1) {
                ExplodeResult::Failed => {}
                ExplodeResult::Success(left_val, Some(right_val)) => {
                    right.add_to_start(right_val);
                    return ExplodeResult::Success(left_val, None);
                }
                result => return result,
            }
            match right.explode_inner(depth + 1) {
                ExplodeResult::Failed => {}
                ExplodeResult::Success(Some(left_val), right_val) => {
                    left.add_to_end(left_val);
                    return ExplodeResult::Success(None, right_val);
                }
                result => return result,
            }
        }
        ExplodeResult::Failed
    }

    fn add_to_start(&mut self, value: i32) {
        match self {
            Snailfish::Number(x) => *x += value,
            Snailfish::Pair(left, _) => left.add_to_start(value),
        }
    }

    fn add_to_end(&mut self, value: i32) {
        match self {
            Snailfish::Number(x) => *x += value,
            Snailfish::Pair(_, right) => right.add_to_end(value),
        }
    }

    pub fn try_split(&mut self) -> bool {
        match self {
            Snailfish::Number(x) if *x >= 10 => {
                let x = *x;
                let left = Snailfish::Number(x / 2);
                let right = Snailfish::Number(x / 2 + (x % 2));
                *self = Snailfish::Pair(Box::new(left), Box::new(right));
                true
            }
            Snailfish::Number(_) => false,
            Snailfish::Pair(left, right) => left.try_split() || right.try_split(),
        }
    }

    pub fn magnitude(&self) -> i32 {
        match self {
            Snailfish::Number(x) => *x,
            Snailfish::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &[Snailfish]) -> i32 {
    let mut result = input[0].clone();
    for number in input.iter().skip(1) {
        result = result.add(number.clone());
    }
    result.magnitude()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Snailfish]) -> i32 {
    let mut max_magnitude = 0;
    for (i, left) in input.iter().enumerate() {
        for (j, right) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            let result = left.clone().add(right.clone());
            max_magnitude = max(max_magnitude, result.magnitude());
        }
    }
    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"".trim();
    }

    #[test]
    fn test_parse_and_display() {
        let input = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let number: Snailfish = input.parse().unwrap();
        assert_eq!(&number.to_string(), input);
    }

    #[test]
    fn test_try_explode() {
        let mut number: Snailfish = "[[[[[9,8],1],2],3],4]".parse().unwrap();
        assert!(number.try_explode());
        assert_eq!(&number.to_string(), "[[[[0,9],2],3],4]");

        let mut number: Snailfish = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
        assert!(number.try_explode());
        assert_eq!(&number.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut number: Snailfish = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
        assert!(number.try_explode());
        assert_eq!(&number.to_string(), "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn test_try_split() {
        let mut number: Snailfish = "10".parse().unwrap();
        assert!(number.try_split());
        assert_eq!(&number.to_string(), "[5,5]");

        let mut number: Snailfish = "11".parse().unwrap();
        assert!(number.try_split());
        assert_eq!(&number.to_string(), "[5,6]");
    }

    #[test]
    fn test_add() {
        let left: Snailfish = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let right: Snailfish = "[1,1]".parse().unwrap();
        let number = left.add(right);
        assert_eq!(&number.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
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
