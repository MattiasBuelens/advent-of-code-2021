use std::cmp::{max, Ordering};

use lazy_static::*;
use regex::Regex;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> TargetArea {
    let captures = INPUT_RE.captures(input).unwrap();
    TargetArea {
        min_x: captures[1].parse().unwrap(),
        max_x: captures[2].parse().unwrap(),
        min_y: captures[3].parse().unwrap(),
        max_y: captures[4].parse().unwrap(),
    }
}

impl TargetArea {
    pub fn contains(&self, pos: &Vector2D) -> bool {
        (self.min_x..=self.max_x).contains(&pos.x())
            && (self.min_y..=self.max_y).contains(&pos.y())
    }
}

#[aoc(day17, part1)]
pub fn part1(target: &TargetArea) -> i32 {
    let mut max_y = None;
    // X velocity must not be beyond target area, otherwise we miss it in the first step
    for vel_x in (0..=target.max_x).chain(target.min_y..=0) {
        // TODO Max bound for Y velocity?
        for vel_y in 0..1000 {
            if let Some(y) = launch(Vector2D::new(vel_x, vel_y), target) {
                max_y = max_y.map(|old_y| max(old_y, y)).or(Some(y))
            }
        }
    }
    max_y.unwrap()
}

fn launch(mut velocity: Vector2D, target: &TargetArea) -> Option<i32> {
    let mut pos = Vector2D::zero();
    let mut max_y = pos.y();
    while pos.y() >= target.min_y {
        max_y = max(max_y, pos.y());
        if target.contains(&pos) {
            // Hit
            return Some(max_y);
        }
        pos += velocity;
        *velocity.x_mut() += match velocity.x().cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0
        };
        *velocity.y_mut() -= 1;
    }
    // Miss
    None
}

#[aoc(day17, part2)]
pub fn part2(target: &TargetArea) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"target area: x=20..30, y=-10..-5";

    #[test]
    fn test_launch() {
        let target = input_generator(&TEST_INPUT);
        assert_eq!(launch(Vector2D::new(7, 2), &target), Some(3));
        assert_eq!(launch(Vector2D::new(6, 3), &target), Some(6));
        assert_eq!(launch(Vector2D::new(9, 0), &target), Some(0));
        assert_eq!(launch(Vector2D::new(17, -4), &target), None);
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 45);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
