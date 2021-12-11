use crate::util::Vector2D;

#[aoc_generator(day11)]
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

#[aoc(day11, part1)]
pub fn part1(octopuses: &[Vec<u8>]) -> i32 {
    let mut octopuses = octopuses.to_vec();
    let mut flashes = 0;
    for _ in 1..=100 {
        flashes += step(&mut octopuses);
    }
    flashes
}

fn step(octopuses: &mut [Vec<u8>]) -> i32 {
    let mut flashes = 0;
    // First, the energy level of each octopus increases by 1.
    for row in octopuses.iter_mut() {
        for octopus in row {
            *octopus += 1;
        }
    }
    // Then, any octopus with an energy level greater than 9 flashes.
    // This process continues as long as new octopuses keep having
    // their energy level increased beyond 9.
    let height = octopuses.len();
    let width = octopuses[0].len();
    loop {
        let prev_flashes = flashes;
        for y in 0..height {
            for x in 0..width {
                let octopus = &mut octopuses[y][x];
                if *octopus > 9 {
                    flashes += 1;
                    // Any octopus that flashed during this step
                    // has its energy level set to 0
                    *octopus = 0;
                    // This increases the energy level of all adjacent octopuses by 1.
                    let pos = Vector2D::new(x as i32, y as i32);
                    for neighbour_pos in get_neighbours(octopuses, pos) {
                        let neighbour =
                            &mut octopuses[neighbour_pos.y() as usize][neighbour_pos.x() as usize];
                        // 0 means it has already flashed during this step, so don't flash again.
                        if *neighbour != 0 {
                            *neighbour += 1;
                        }
                    }
                }
            }
        }
        if flashes == prev_flashes {
            break;
        }
    }
    flashes
}

fn get_neighbours(map: &[Vec<u8>], pos: Vector2D) -> Vec<Vector2D> {
    [
        pos + Vector2D::new(-1, -1),
        pos + Vector2D::new(-1, 0),
        pos + Vector2D::new(-1, 1),
        pos + Vector2D::new(0, -1),
        pos + Vector2D::new(0, 1),
        pos + Vector2D::new(1, -1),
        pos + Vector2D::new(1, 0),
        pos + Vector2D::new(1, 1),
    ]
    .into_iter()
    .filter(|pos| {
        pos.y() >= 0 && pos.y() < map.len() as i32 && pos.x() >= 0 && pos.x() < map[0].len() as i32
    })
    .collect()
}

#[allow(unused)]
fn print_grid(octopuses: &[Vec<u8>]) {
    octopuses.iter().for_each(|row| {
        row.iter().for_each(|octopus| print!("{}", octopus));
        println!();
    });
}

#[aoc(day11, part2)]
pub fn part2(octopuses: &[Vec<u8>]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
