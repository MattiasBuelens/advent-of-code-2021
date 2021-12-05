use std::cmp::{max, min};
use std::collections::HashMap;

use crate::util::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Line(Vector2D, Vector2D);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let [start, end]: [Vector2D; 2] = line
                .splitn(2, " -> ")
                .map(|point| {
                    let [x, y]: [i32; 2] = point
                        .splitn(2, ',')
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap();
                    Vector2D::new(x, y)
                })
                .collect::<Vec<Vector2D>>()
                .try_into()
                .unwrap();
            Line(start, end)
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(lines: &[Line]) -> usize {
    let mut grid = HashMap::<Vector2D, usize>::new();
    for Line(start, end) in lines {
        if start.x() == end.x() {
            let min_y = min(start.y(), end.y());
            let max_y = max(start.y(), end.y());
            for y in min_y..=max_y {
                grid.entry(Vector2D::new(start.x(), y))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        } else if start.y() == end.y() {
            let min_x = min(start.x(), end.x());
            let max_x = max(start.x(), end.x());
            for x in min_x..=max_x {
                grid.entry(Vector2D::new(x, start.y()))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    grid.values().filter(|&&count| count >= 2).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
