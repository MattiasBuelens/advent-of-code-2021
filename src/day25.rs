use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cucumbers: HashMap<Vector2D, SeaCucumber>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeaCucumber {
    East,
    South,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Grid {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let cucumbers = input
        .lines()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.chars().enumerate().flat_map(move |(x, c)| {
                let pos = Vector2D::new(x as i32, y as i32);
                match c {
                    '>' => Some((pos, SeaCucumber::East)),
                    'v' => Some((pos, SeaCucumber::South)),
                    '.' => None,
                    c => panic!("invalid character: {}", c),
                }
            })
        })
        .collect();
    Grid {
        width,
        height,
        cucumbers,
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &Grid) -> i32 {
    todo!()
}

#[aoc(day25, part2)]
pub fn part2(input: &Grid) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            .trim();
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
