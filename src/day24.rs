use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    Input(Variable),
    Add(Variable, Operand),
    Mul(Variable, Operand),
    Div(Variable, Operand),
    Mod(Variable, Operand),
    Eql(Variable, Operand),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let operator = parts.next().unwrap();
        let first = parts.next().unwrap();
        Ok(match operator {
            "inp" => Instruction::Input(first.parse()?),
            "add" => Instruction::Add(first.parse()?, parts.next().unwrap().parse()?),
            "mul" => Instruction::Mul(first.parse()?, parts.next().unwrap().parse()?),
            "div" => Instruction::Div(first.parse()?, parts.next().unwrap().parse()?),
            "mod" => Instruction::Mod(first.parse()?, parts.next().unwrap().parse()?),
            "eql" => Instruction::Eql(first.parse()?, parts.next().unwrap().parse()?),
            _ => panic!("invalid instruction"),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operand {
    Var(Variable),
    Num(i32),
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(var) = s.parse::<Variable>() {
            Ok(Operand::Var(var))
        } else {
            Ok(Operand::Num(s.parse().expect("invalid number")))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Variable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Variable::W),
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    todo!()
}

#[aoc(day24, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
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
