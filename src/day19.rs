use std::str::FromStr;

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
