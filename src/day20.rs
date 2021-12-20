use std::collections::HashSet;

use crate::util::Vector2D;

pub type Image = HashSet<Vector2D>;
pub type Algorithm = [bool; 512];
pub type Input = (Algorithm, Image);

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let image = image
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Vector2D::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    (algorithm, image)
}

#[aoc(day20, part1)]
pub fn part1((algorithm, image): &Input) -> i32 {
    todo!()
}

#[aoc(day20, part2)]
pub fn part2((algorithm, image): &Input) -> i32 {
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
