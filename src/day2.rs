use std::str::FromStr;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, amount) = s.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().expect("invalid amount");
        Ok(match cmd {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic!("invalid command"),
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> i32 {
    let mut pos = Vector2D::zero();
    for cmd in input {
        match cmd {
            Command::Forward(amount) => {
                *pos.x_mut() += amount;
            }
            Command::Down(amount) => {
                *pos.y_mut() += amount;
            }
            Command::Up(amount) => {
                *pos.y_mut() -= amount;
            }
        }
    }
    pos.x() * pos.y()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Command]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 150);
    }
}
