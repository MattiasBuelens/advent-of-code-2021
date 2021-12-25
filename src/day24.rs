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

/*
   The MONAD program consists of 14 phases, where each phase processes one input with a similar
   set of instructions matching the following regular expression:

   inp w
   mul x 0
   add x z
   mod x 26
   div z (1|26)     // 1   1  1 26  1  1  1 26 26  1  26 26  26 26   (7x1, 7x26)
   add x (-?\d+)    // 14 12 11 -4 10 10 15 -9 -9 12 -15 -7 -10  0
   eql x           //  1   2  3  4  3  4  5  4  3  4   3  2   1  0  = log(Z)/log(26) after step
   eql x 0
   mul y 0
   add y 25
   mul y x
   add y 1
   mul z y
   mul y 0
   add y w
   add y (-?\d+)    // 7 4 8 1 5 14 12 10 5 7 6 8 4 6
   mul y x
   add z y

   Decompiled:

   w = read()
   x = z % 26
   z = z  OR  z = z / 26
   x += SOME_NUMBER
   x = (x != w)                // 0 or 1
   z *= (25 * x) + 1
   z += (w + SOME_OTHER_NUMBER) * x

   Decompiled further:

   w = read()
   x = (z % 26) + SOME_NUMBER
   z = z  OR  z = z / 26
   if (x != w) {
     z = (26 * z) + (w + SOME_OTHER_NUMBER)
   }

   In the last phase, if x != w:
   0 == (26 * z) + (w + 6)
   but this is impossible: (w + 6) must be a multiple of 26, but w is between 1 and 9.
   Thus, in the last phase, x == w, and z must already be 0.
   Earlier, z = z / 26, so at the start -25 <= z <= 25, so it rounds to 0.
   At the same time, we have:
   x = (z % 26) + 0
   We must have x == w, so at the start (z % 26) == w, or z == w.
   Thus, at the start of the last step, z = last digit, and therefore 1 <= z <= 9.

   In phases 1 to 3, we divide z by 1, so it cannot decrease. At the start of step 4, we have:
   z = ((a + 7) * 26 + b + 4) * 26) + c + 8
   z = 26^2 * a + 26^1 * b + c + 4844
   z = c + 8 (mod 26)

   In order to fail the if-check, we must have:
   x = (c + 8) % 26 + (-4)
   x == w
   Case-by-case analysis:
    * c = 1: x = w = 5
    * c = 2: x = w = 6
    * c = 3: x = w = 7
    * c = 4: x = w = 8
    * c = 5: x = w = 9
    * c = 6: x = 10, no valid digit for w

*/

fn solve(steps: &[Step], ws: Vec<i32>, zs: Vec<i32>) -> Vec<Vec<i32>> {
    let (step, steps) = match steps.split_first() {
        Some(x) => x,
        None => return vec![ws],
    };
    match step.div {
        1 => {
            // Add new digit
            let z = zs.last().expect("missing z");
            (1..=9)
                .flat_map(|w| {
                    let mut ws = ws.clone();
                    ws.push(w);
                    let mut zs = ws.clone();
                    let z = (26 * z) + (w + step.accum);
                    zs.push(z);
                    solve(steps, ws, zs)
                })
                .collect()
        }
        26 => {
            // Match with previous Z
            let z = zs.last().expect("missing z");
            let w = z % 26 + step.check;
            if (1..=9).contains(&w) {
                // Found a valid digit to make the check pass
                let mut ws = ws.clone();
                ws.push(w);
                let mut zs = zs.clone();
                let z = z / 26;
                zs.push(z);
                solve(steps, ws, zs)
            } else {
                // Not a valid digit, no solutions
                Vec::new()
            }
        }
        div => panic!("unexpected divisor {}", div),
    }
}

#[derive(Debug, Copy, Clone)]
struct Step {
    div: i32,
    check: i32,
    accum: i32,
}

fn extract_step(program: &[Instruction]) -> Step {
    if let &Instruction::Div(Variable::Z, Operand::Num(div)) = &program[4] {
        if let &Instruction::Add(Variable::X, Operand::Num(check)) = &program[5] {
            if let &Instruction::Add(Variable::Y, Operand::Num(accum)) = &program[15] {
                return Step { div, check, accum };
            }
        }
    }
    panic!("malformed step");
}

#[aoc(day24, part1)]
pub fn part1(input: &[Instruction]) -> u64 {
    let steps = input
        .chunks_exact(18)
        .map(|x| extract_step(x))
        .collect::<Vec<_>>();
    let solutions = solve(&steps, Vec::new(), vec![0]);

    let max_solution = solutions.into_iter().max().expect("no solutions");
    max_solution.iter().fold(0u64, |acc, digit| acc * 10 + (*digit as u64))
}

#[aoc(day24, part2)]
pub fn part2(input: &[Instruction]) -> u64 {
    let steps = input
        .chunks_exact(18)
        .map(|x| extract_step(x))
        .collect::<Vec<_>>();
    let solutions = solve(&steps, Vec::new(), vec![0]);

    let min_solution = solutions.into_iter().min().expect("no solutions");
    min_solution.iter().fold(0u64, |acc, digit| acc * 10 + (*digit as u64))
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
div w 10"
                .trim(),
        );
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
