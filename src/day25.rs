use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Grid {
    width: i32,
    height: i32,
    cucumbers: HashMap<Vector2D, SeaCucumber>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeaCucumber {
    East,
    South,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Grid {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
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

impl Grid {
    fn step(&mut self) -> bool {
        let east_moved = self.move_herd(SeaCucumber::East);
        let south_moved = self.move_herd(SeaCucumber::South);
        east_moved || south_moved
    }

    fn move_herd(&mut self, herd: SeaCucumber) -> bool {
        let cucumbers = self.cucumbers.clone();
        let mut did_move = false;
        for (pos, cucumber) in cucumbers.iter() {
            if *cucumber != herd {
                continue;
            }
            let mut next_pos = *pos
                + match cucumber {
                    SeaCucumber::East => Vector2D::new(1, 0),
                    SeaCucumber::South => Vector2D::new(0, 1),
                };
            if next_pos.x() == self.width {
                *next_pos.x_mut() = 0;
            }
            if next_pos.y() == self.height {
                *next_pos.y_mut() = 0;
            }
            if !cucumbers.contains_key(&next_pos) {
                did_move = true;
                self.cucumbers.remove(pos);
                self.cucumbers.insert(next_pos, *cucumber);
            }
        }
        did_move
    }
}

#[aoc(day25, part1)]
pub fn part1(grid: &Grid) -> i32 {
    let mut grid = grid.clone();
    for i in 1.. {
        if !grid.step() {
            return i;
        }
    }
    panic!("cannot happen")
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
        assert_eq!(part1(&input), 58);
    }
}
