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
    let mut low_points: Vec<u8> = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            let neighbours = get_neighbours(map, x, y);
            if neighbours
                .iter()
                .all(|neighbour_height| height < neighbour_height)
            {
                low_points.push(*height);
            }
        }
    }
    low_points.iter().map(|&x| (x as i32) + 1).sum()
}

fn get_neighbours(map: &HeightMap, x: usize, y: usize) -> Vec<u8> {
    [
        x.checked_sub(1).map(|x| (x, y)),
        x.checked_add(1).map(|x| (x, y)),
        y.checked_sub(1).map(|y| (x, y)),
        y.checked_add(1).map(|y| (x, y)),
    ]
    .into_iter()
    .filter_map(|pos| pos)
    .filter_map(|(x, y)| map.get(y)?.get(x))
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
