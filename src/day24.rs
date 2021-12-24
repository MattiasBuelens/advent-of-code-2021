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

struct Alu {
    program: Vec<Instruction>,
    variables: [i32; 4],
}

impl Alu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            variables: [0; 4],
        }
    }

    fn run(&mut self, input: &[i32]) {
        let mut input = input.into_iter().copied();
        for instruction in self.program.clone() {
            self.step(instruction, &mut input);
        }
    }

    fn step(&mut self, instruction: Instruction, input: &mut impl Iterator<Item = i32>) {
        match instruction {
            Instruction::Input(a) => {
                *self.var_mut(a) = input.next().expect("missing input");
            }
            Instruction::Add(a, b) => {
                *self.var_mut(a) += self.get(b);
            }
            Instruction::Mul(a, b) => {
                *self.var_mut(a) *= self.get(b);
            }
            Instruction::Div(a, b) => {
                *self.var_mut(a) = *self.var(a) / self.get(b);
            }
            Instruction::Mod(a, b) => {
                *self.var_mut(a) = *self.var(a) % self.get(b);
            }
            Instruction::Eql(a, b) => {
                *self.var_mut(a) = if *self.var(a) == self.get(b) { 1 } else { 0 };
            }
        }
    }

    fn get(&self, operand: Operand) -> i32 {
        match operand {
            Operand::Var(var) => *self.var(var),
            Operand::Num(val) => val,
        }
    }

    fn var(&self, variable: Variable) -> &i32 {
        &self.variables[Self::var_index(variable)]
    }

    fn var_mut(&mut self, variable: Variable) -> &mut i32 {
        &mut self.variables[Self::var_index(variable)]
    }

    fn var_index(variable: Variable) -> usize {
        match variable {
            Variable::W => 0,
            Variable::X => 1,
            Variable::Y => 2,
            Variable::Z => 3,
        }
    }
}

#[aoc_generator(day24)]
pub fn parse_program(input: &str) -> Vec<Instruction> {
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
    fn test_negate() {
        let program = parse_program(
            r"
inp x
mul x -1"
                .trim(),
        );
        let mut alu = Alu::new(program);
        alu.run(&[123]);
        assert_eq!(*alu.var(Variable::X), -123);
    }

    #[test]
    fn test_three_times() {
        let program = parse_program(
            r"
inp z
inp x
mul z 3
eql z x"
                .trim(),
        );
        let mut alu = Alu::new(program);
        alu.run(&[2, 6]);
        assert_eq!(*alu.var(Variable::Z), 1);
        alu.run(&[2, 7]);
        assert_eq!(*alu.var(Variable::Z), 0);
    }

    #[test]
    fn test_binary() {
        let program = parse_program(
            r"
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"
                .trim(),
        );
        let mut alu = Alu::new(program);
        alu.run(&[0b1010]);
        assert_eq!(*alu.var(Variable::W), 1);
        assert_eq!(*alu.var(Variable::X), 0);
        assert_eq!(*alu.var(Variable::Y), 1);
        assert_eq!(*alu.var(Variable::Z), 0);
    }

    #[test]
    fn test_div() {
        let program = parse_program(
            r"
inp w
div w 10".trim());
        let mut alu = Alu::new(program);
        alu.run(&[0]);
        assert_eq!(*alu.var(Variable::W), 0);
        alu.run(&[10]);
        assert_eq!(*alu.var(Variable::W), 1);
        alu.run(&[-10]);
        assert_eq!(*alu.var(Variable::W), -1);
        alu.run(&[11]);
        assert_eq!(*alu.var(Variable::W), 1);
        alu.run(&[29]);
        assert_eq!(*alu.var(Variable::W), 2);
        alu.run(&[-11]);
        assert_eq!(*alu.var(Variable::W), -1);
        alu.run(&[-29]);
        assert_eq!(*alu.var(Variable::W), -2);
    }

    #[test]
    fn test_part1() {
        let input = parse_program(&TEST_INPUT);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = parse_program(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
