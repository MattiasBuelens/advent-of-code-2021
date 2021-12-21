use std::collections::HashSet;

use crate::util::Vector2D;

#[derive(Debug, Clone)]
pub struct Image {
    contents: HashSet<Vector2D>,
    // Bounds for finite region
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    // Infinitely repeating pixel around finite bounds
    infinite: bool,
}

pub type Algorithm = [bool; 512];
pub type Input = (Algorithm, Image);

impl Image {
    pub fn new(contents: HashSet<Vector2D>) -> Image {
        let min_x = contents.iter().map(|pos| pos.x()).min().unwrap();
        let max_x = contents.iter().map(|pos| pos.x()).max().unwrap();
        let min_y = contents.iter().map(|pos| pos.y()).min().unwrap();
        let max_y = contents.iter().map(|pos| pos.y()).max().unwrap();
        Self {
            contents,
            min_x,
            max_x,
            min_y,
            max_y,
            infinite: false,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c == '#')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let image_contents = image
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
    let image = Image::new(image_contents);
    (algorithm, image)
}

impl Image {
    pub fn enhance(self, algorithm: &Algorithm) -> Self {
        let mut contents = HashSet::new();
        // The finite region expands by at most 1 pixel
        let min_x = self.min_x - 1;
        let max_x = self.max_x + 1;
        let min_y = self.min_y - 1;
        let max_y = self.max_y + 1;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Vector2D::new(x, y);
                let index = self.get_enhance_index(pos);
                if algorithm[index] {
                    contents.insert(pos);
                }
            }
        }
        // At infinity, either all pixels are lit (511), or all are off (0)
        let infinite = if self.infinite {
            algorithm[511]
        } else {
            algorithm[0]
        };
        Self {
            contents,
            min_x,
            max_x,
            min_y,
            max_y,
            infinite,
        }
    }

    fn get_enhance_index(&self, pos: Vector2D) -> usize {
        let digits = [
            pos + Vector2D::new(-1, -1),
            pos + Vector2D::new(0, -1),
            pos + Vector2D::new(1, -1),
            pos + Vector2D::new(-1, 0),
            pos,
            pos + Vector2D::new(1, 0),
            pos + Vector2D::new(-1, 1),
            pos + Vector2D::new(0, 1),
            pos + Vector2D::new(1, 1),
        ]
        .into_iter()
        .map(|pos| if self.get_pixel(&pos) { '1' } else { '0' })
        .collect::<String>();
        usize::from_str_radix(&digits, 2).unwrap()
    }

    fn get_pixel(&self, pos: &Vector2D) -> bool {
        if (self.min_x..=self.max_x).contains(&pos.x())
            && (self.min_y..=self.max_y).contains(&pos.y())
        {
            self.contents.contains(pos)
        } else {
            self.infinite
        }
    }
}

#[aoc(day20, part1)]
pub fn part1((algorithm, image): &Input) -> usize {
    let image = image.clone();
    let image = image.enhance(algorithm);
    let image = image.enhance(algorithm);
    assert!(!image.infinite);
    image.contents.len()
}

#[aoc(day20, part2)]
pub fn part2((algorithm, image): &Input) -> usize {
    let mut image = image.clone();
    for _ in 1..=50 {
        image = image.enhance(algorithm);
    }
    assert!(!image.infinite);
    image.contents.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 3351);
    }
}
