use std::cmp::Ordering;
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

fn sign(x: i32) -> i32 {
    match x.cmp(&0) {
        Ordering::Equal => 0,
        Ordering::Greater => 1,
        Ordering::Less => -1,
    }
}

fn count_overlaps(lines: &[Line], diagonals: bool) -> usize {
    let mut grid = HashMap::<Vector2D, usize>::new();
    for &Line(start, end) in lines {
        let diff = end - start;
        if !diagonals && diff.x() != 0 && diff.y() != 0 {
            continue;
        }
        let step = Vector2D::new(sign(diff.x()), sign(diff.y()));
        let mut pos = start;
        while pos != end {
            grid.entry(pos).and_modify(|count| *count += 1).or_insert(1);
            pos += step;
        }
        grid.entry(end).and_modify(|count| *count += 1).or_insert(1);
    }
    grid.values().filter(|&&count| count >= 2).count()
}

#[aoc(day5, part1)]
pub fn part1(lines: &[Line]) -> usize {
    count_overlaps(lines, false)
}

#[aoc(day5, part2)]
pub fn part2(lines: &[Line]) -> usize {
    count_overlaps(lines, true)
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
        assert_eq!(part2(&input), 12);
    }
}
