use pathfinding::directed::dijkstra::*;

use crate::util::Vector2D;

type Cave = Vec<Vec<u32>>;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Cave {
    input
        .lines()
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(cave: &Cave) -> u32 {
    let start = Vector2D::new(0, 0);
    let goal = Vector2D::new(cave[0].len() as i32 - 1, cave.len() as i32 - 1);

    let (_path, cost) = dijkstra(
        &start,
        |pos| -> Vec<(Vector2D, u32)> {
            get_neighbours(cave, *pos)
                .into_iter()
                .map(|pos| {
                    let risk = cave[pos.y() as usize][pos.x() as usize];
                    (pos, risk)
                })
                .collect()
        },
        |pos| pos == &goal,
    )
    .unwrap();

    cost
}

fn get_neighbours(cave: &Cave, pos: Vector2D) -> Vec<Vector2D> {
    [
        pos + Vector2D::new(-1, 0),
        pos + Vector2D::new(0, -1),
        pos + Vector2D::new(0, 1),
        pos + Vector2D::new(1, 0),
    ]
    .into_iter()
    .filter(|pos| {
        pos.y() >= 0
            && pos.y() < cave.len() as i32
            && pos.x() >= 0
            && pos.x() < cave[0].len() as i32
    })
    .collect()
}

#[aoc(day15, part2)]
pub fn part2(cave: &Cave) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
