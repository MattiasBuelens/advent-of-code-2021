use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::*;

use crate::util::Vector3D;

#[derive(Debug, Clone)]
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
struct Basis {
    x: Vector3D,
    y: Vector3D,
    z: Vector3D,
}

impl Basis {
    pub fn apply(self, pos: Vector3D) -> Vector3D {
        // https://en.wikipedia.org/wiki/Change_of_basis
        Vector3D::new(
            self.x.x() * pos.x() + self.y.x() * pos.y() + self.z.x() * pos.z(),
            self.x.y() * pos.x() + self.y.y() * pos.y() + self.z.y() * pos.z(),
            self.x.z() * pos.x() + self.y.z() * pos.y() + self.z.z() * pos.z(),
        )
    }
}

fn get_rotations() -> [Basis; 24] {
    let all_axis = [
        Vector3D::new(1, 0, 0),
        Vector3D::new(0, 1, 0),
        Vector3D::new(0, 0, 1),
    ];
    all_axis
        .into_iter()
        .cartesian_product(all_axis)
        .filter(|(x, y)| x != y)
        .flat_map(|(x_unit, y_unit)| {
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

lazy_static! {
    static ref ROTATIONS: [Basis; 24] = get_rotations();
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Vec<Report> {
    input
        .split("\n\n")
        .map(|line| line.parse().unwrap())
        .collect()
}

struct ScannerState {
    position: Vector3D,
    rotation: Basis,
}

impl ScannerState {
    pub fn transform(&self, relative_beacon: Vector3D) -> Vector3D {
        self.rotation.apply(relative_beacon) + self.position
    }
}

fn step(
    beacons: HashSet<Vector3D>,
    reports: Vec<&Report>,
) -> Option<(HashSet<Vector3D>, Vec<&Report>)> {
    if reports.is_empty() {
        // All done!
        return Some((beacons, reports));
    }
    for report in reports.iter() {
        // Pick a matching beacon
        for &relative_beacon in report.beacons.iter() {
            for &absolute_beacon in beacons.iter() {
                // Pick an orientation
                for &rotation in ROTATIONS.iter() {
                    // position + (relative beacon * rotation) = absolute beacon
                    let position = absolute_beacon - rotation.apply(relative_beacon);
                    let state = ScannerState { position, rotation };
                    let matching_beacons = report
                        .beacons
                        .iter()
                        .map(|pos| state.transform(*pos))
                        .filter(|pos| beacons.contains(&pos))
                        .count();
                    debug_assert!(matching_beacons >= 1, "at least one beacon should match");
                    if matching_beacons >= 12 {
                        // Found a match!
                        // Add new beacons, and recurse with remaining reports
                        let mut new_beacons = beacons.clone();
                        new_beacons.extend(report.beacons.iter().map(|pos| state.transform(*pos)));
                        let remaining_reports = reports
                            .iter()
                            .filter(|x| x.scanner_id != report.scanner_id)
                            .cloned()
                            .collect();
                        if let result @ Some(_) = step(new_beacons, remaining_reports) {
                            return result;
                        }
                    }
                }
            }
        }
    }
    // No matching report found, discard this
    None
}

#[aoc(day19, part1)]
pub fn part1(reports: &[Report]) -> usize {
    let mut reports = reports.to_vec();
    // Set scanner 0 at (0,0,0) with default orientation
    let first_state = ScannerState {
        position: Vector3D::zero(),
        rotation: ROTATIONS[0],
    };
    let first_beacons = reports
        .remove(0)
        .beacons
        .into_iter()
        .map(|pos| first_state.transform(pos))
        .collect();
    // Solve
    let (beacons, _) = step(first_beacons, reports.iter().collect()).expect("no solution");
    // Count number of beacons
    beacons.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Report]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("sample/day19.txt");

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 79);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
