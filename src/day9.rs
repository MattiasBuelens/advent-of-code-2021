use crate::util::Vector2D;

type HeightMap = Vec<Vec<u8>>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> HeightMap {
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
            let neighbours = get_neighbours(map, pos);
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

fn get_neighbours(map: &HeightMap, pos: Vector2D) -> Vec<u8> {
    [
        pos + Vector2D::new(-1, 0),
        pos + Vector2D::new(1, 0),
        pos + Vector2D::new(0, -1),
        pos + Vector2D::new(0, 1),
    ]
    .into_iter()
    .filter_map(|pos| map.get(pos.y() as usize)?.get(pos.x() as usize))
    .copied()
    .collect()
}

#[aoc(day9, part2)]
pub fn part2(map: &HeightMap) -> i32 {
    todo!()
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
        assert_eq!(part2(&input), 0);
    }
}
