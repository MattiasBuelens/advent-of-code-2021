use std::str::FromStr;

use itertools::Itertools;

use crate::util::Vector3D;

#[derive(Debug)]
pub struct Report {
    scanner_id: u8,
    beacons: Vec<Vector3D>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let scanner_id = lines
            .next()
            .unwrap()
            .strip_prefix("--- scanner ")
            .unwrap()
            .strip_suffix(" ---")
            .unwrap()
            .parse()
            .unwrap();
        let beacons = lines
            .map(|line| {
                let coords: [i32; 3] = line
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                Vector3D::from(coords)
            })
            .collect();
        Ok(Report {
            scanner_id,
            beacons,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn unit(&self) -> Vector3D {
        match self {
            Axis::X => Vector3D::new(1, 0, 0),
            Axis::Y => Vector3D::new(0, 1, 0),
            Axis::Z => Vector3D::new(0, 0, 1),
        }
    }
}

#[derive(Debug)]
struct Basis {
    x: Vector3D,
    y: Vector3D,
    z: Vector3D,
}

impl Basis {
    pub fn apply(&self, pos: Vector3D) -> Vector3D {
        // https://en.wikipedia.org/wiki/Change_of_basis
        Vector3D::new(
            self.x.x() * pos.x() + self.y.x() * pos.y() + self.z.x() * pos.z(),
            self.x.y() * pos.x() + self.y.y() * pos.y() + self.z.y() * pos.z(),
            self.x.z() * pos.x() + self.y.z() * pos.y() + self.z.z() * pos.z(),
        )
    }
}

fn get_rotations() -> [Basis; 24] {
    let all_axis = [Axis::X, Axis::Y, Axis::Z];
    all_axis
        .into_iter()
        .cartesian_product(all_axis)
        .filter(|(x, y)| x != y)
        .flat_map(|(x_axis, y_axis)| {
            let x_unit = x_axis.unit();
            let y_unit = y_axis.unit();
            [x_unit, -x_unit]
                .into_iter()
                .cartesian_product([y_unit, -y_unit])
                .map(|(x, y)| {
                    let z = x.cross_product(y);
                    Basis { x, y, z }
                })
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Report> {
    input
        .split("\n\n")
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day19, part1)]
pub fn part1(input: &[Report]) -> i32 {
    todo!()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Report]) -> i32 {
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
