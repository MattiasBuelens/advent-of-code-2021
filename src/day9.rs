use std::collections::{HashSet, VecDeque};

use crate::util::Vector2D;

type HeightMap = [Vec<u8>];

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(map: &HeightMap) -> i32 {
    let low_points = get_low_points(map);
    low_points
        .iter()
        .map(|pos| map[pos.y() as usize][pos.x() as usize] as i32 + 1)
        .sum()
}

fn get_low_points(map: &HeightMap) -> Vec<Vector2D> {
    let mut low_points: Vec<Vector2D> = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let pos = Vector2D::new(x as i32, y as i32);
            let neighbours = get_neighbour_heights(map, pos);
            if neighbours
                .iter()
                .all(|neighbour_height| height < neighbour_height)
            {
                low_points.push(pos);
            }
        }
    }
    low_points
}

fn get_neighbours(map: &HeightMap, pos: Vector2D) -> Vec<Vector2D> {
    pos.neighbours()
        .filter(|pos| {
            pos.y() >= 0
                && pos.y() < map.len() as i32
                && pos.x() >= 0
                && pos.x() < map[0].len() as i32
        })
        .collect()
}

fn get_neighbour_heights(map: &HeightMap, pos: Vector2D) -> Vec<u8> {
    get_neighbours(map, pos)
        .into_iter()
        .map(|pos| map[pos.y() as usize][pos.x() as usize])
        .collect()
}

#[aoc(day9, part2)]
pub fn part2(map: &HeightMap) -> i32 {
    let mut basin_sizes = get_low_points(map)
        .into_iter()
        .map(|low_point| get_basin_size(map, low_point))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    basin_sizes.into_iter().take(3).product()
}

fn get_basin_size(map: &HeightMap, low_point: Vector2D) -> i32 {
    let mut visited = HashSet::<Vector2D>::new();
    let mut queue = VecDeque::<Vector2D>::new();
    queue.push_back(low_point);
    while let Some(pos) = queue.pop_front() {
        visited.insert(pos);
        for neighbour_pos in get_neighbours(map, pos) {
            if visited.contains(&neighbour_pos) || queue.contains(&neighbour_pos) {
                continue;
            }
            let height = map[neighbour_pos.y() as usize][neighbour_pos.x() as usize];
            if height < 9 {
                queue.push_back(neighbour_pos);
            }
        }
    }
    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
2199943210
3987894921
9856789892
8767896789
9899965678
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 1134);
    }
}
