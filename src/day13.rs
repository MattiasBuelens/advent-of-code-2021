use std::collections::HashSet;

use crate::util::Vector2D;

pub type Paper = HashSet<Vector2D>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Fold {
    AlongX(i32),
    AlongY(i32),
}

pub type Input = (Paper, Vec<Fold>);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let (paper, folds) = input.split_once("\n\n").unwrap();
    let paper = paper
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = folds
        .lines()
        .map(|line| {
            let (fold_along, coord) = line.split_once('=').unwrap();
            let coord = coord.parse().unwrap();
            match fold_along {
                "fold along x" => Fold::AlongX(coord),
                "fold along y" => Fold::AlongY(coord),
                _ => panic!("unexpected input: {}", line),
            }
        })
        .collect();
    (paper, folds)
}

fn fold_paper(paper: &Paper, fold: Fold) -> Paper {
    paper.iter().map(|pos| fold_dot(*pos, fold)).collect()
}

fn fold_dot(mut pos: Vector2D, fold: Fold) -> Vector2D {
    match fold {
        Fold::AlongX(fold_x) if pos.x() > fold_x => {
            *pos.x_mut() = 2 * fold_x - pos.x();
        }
        Fold::AlongY(fold_y) if pos.y() > fold_y => {
            *pos.y_mut() = 2 * fold_y - pos.y();
        }
        _ => {}
    }
    pos
}

fn print_paper(paper: &Paper) {
    let width = paper.iter().map(|pos| pos.x()).max().unwrap();
    let height = paper.iter().map(|pos| pos.y()).max().unwrap();
    for y in 0..=height {
        for x in 0..=width {
            match paper.contains(&Vector2D::new(x, y)) {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!();
    }
}

#[aoc(day13, part1)]
pub fn part1((paper, folds): &Input) -> usize {
    let paper = fold_paper(paper, folds[0]);
    paper.len()
}

#[aoc(day13, part2)]
pub fn part2((paper, folds): &Input) -> i32 {
    let mut paper = paper.clone();
    for fold in folds {
        paper = fold_paper(&paper, *fold);
    }
    print_paper(&paper);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 17);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        part2(&input);
    }
}
