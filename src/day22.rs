use std::collections::HashSet;
use std::ops::RangeInclusive;

use itertools::Either;
use lazy_static::*;
use regex::Regex;

use crate::util::Vector3D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cuboid {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct RebootStep(bool, Cuboid);

lazy_static! {
    static ref INPUT_RE: Regex =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<RebootStep> {
    input
        .lines()
        .map(|line| {
            let captures = INPUT_RE.captures(line).unwrap();
            let on = &captures[1] == "on";
            let cuboid = Cuboid {
                x: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
                y: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
                z: (captures[6].parse().unwrap(), captures[7].parse().unwrap()),
            };
            RebootStep(on, cuboid)
        })
        .collect()
}

impl Cuboid {
    pub fn xs(&self) -> RangeInclusive<i32> {
        self.x.0..=self.x.1
    }

    pub fn ys(&self) -> RangeInclusive<i32> {
        self.y.0..=self.y.1
    }

    pub fn zs(&self) -> RangeInclusive<i32> {
        self.z.0..=self.z.1
    }

    pub fn overlaps(&self, other: &Cuboid) -> bool {
        self.x.0 <= other.x.1
            && other.x.0 <= self.x.1
            && self.y.0 <= other.y.1
            && other.y.0 <= self.y.1
            && self.z.0 <= other.z.1
            && other.z.0 <= self.z.1
    }

    pub fn top_left(&self) -> Vector3D {
        Vector3D::new(self.x.0, self.y.0, self.z.0)
    }

    pub fn bottom_right(&self) -> Vector3D {
        Vector3D::new(self.x.1, self.y.1, self.z.1)
    }
}

#[aoc(day22, part1)]
pub fn part1(steps: &[RebootStep]) -> usize {
    let mut cubes = HashSet::<Vector3D>::new();
    for RebootStep(on, cuboid) in steps {
        for x in cuboid.xs() {
            if x.abs() > 50 {
                continue;
            }
            for y in cuboid.ys() {
                if y.abs() > 50 {
                    continue;
                }
                for z in cuboid.zs() {
                    if z.abs() > 50 {
                        continue;
                    }
                    let pos = Vector3D::new(x, y, z);
                    if *on {
                        cubes.insert(pos);
                    } else {
                        cubes.remove(&pos);
                    }
                }
            }
        }
    }
    cubes.len()
}

impl Cuboid {
    pub fn split(self, pos: Vector3D) -> impl Iterator<Item = Self> {
        self.split_x(pos.x())
            .into_iter()
            .flat_map(move |cuboid| cuboid.split_y(pos.y()).into_iter())
            .flat_map(move |cuboid| cuboid.split_z(pos.z()).into_iter())
    }

    /// Split the cuboid into two halves such that one half only contains points < X,
    /// and the other half only contains points >= X.
    pub fn split_x(self, x: i32) -> Either<[Self; 1], [Self; 2]> {
        if self.x.0 < x && x <= self.x.1 {
            Either::Right([
                Cuboid {
                    x: (self.x.0, x - 1),
                    ..self
                },
                Cuboid {
                    x: (x, self.x.1),
                    ..self
                },
            ])
        } else {
            Either::Left([self])
        }
    }

    pub fn split_y(self, y: i32) -> Either<[Self; 1], [Self; 2]> {
        if self.y.0 < y && y <= self.y.1 {
            Either::Right([
                Cuboid {
                    y: (self.y.0, y - 1),
                    ..self
                },
                Cuboid {
                    y: (y, self.y.1),
                    ..self
                },
            ])
        } else {
            Either::Left([self])
        }
    }

    pub fn split_z(self, z: i32) -> Either<[Self; 1], [Self; 2]> {
        if self.z.0 < z && z <= self.z.1 {
            Either::Right([
                Cuboid {
                    z: (self.z.0, z - 1),
                    ..self
                },
                Cuboid {
                    z: (z, self.z.1),
                    ..self
                },
            ])
        } else {
            Either::Left([self])
        }
    }

    pub fn volume(&self) -> u64 {
        let x = (self.x.1 - self.x.0 + 1) as u64;
        let y = (self.y.1 - self.y.0 + 1) as u64;
        let z = (self.z.1 - self.z.0 + 1) as u64;
        x * y * z
    }
}

#[aoc(day22, part2)]
pub fn part2(steps: &[RebootStep]) -> u64 {
    let mut cuboids = Vec::<Cuboid>::new();
    for RebootStep(on, reboot_cuboid) in steps {
        let mut splits = vec![];
        cuboids.retain(|cuboid| {
            // If cuboid overlaps with current step, remove it from the list
            // and store the split parts that are *outside* of the current step
            if cuboid.overlaps(reboot_cuboid) {
                let remainder = cuboid
                    .split(reboot_cuboid.top_left())
                    .flat_map(|x| x.split(reboot_cuboid.bottom_right() + Vector3D::new(1, 1, 1)))
                    .filter(|x| !x.overlaps(reboot_cuboid));
                splits.extend(remainder);
                false
            } else {
                true
            }
        });
        // Add the split parts back
        cuboids.extend(splits);
        // Add the step's cuboid if it's on
        if *on {
            cuboids.push(*reboot_cuboid);
        }
    }
    cuboids.iter().map(|x| x.volume()).sum()
}

fn is_initialization_step(step: &RebootStep) -> bool {
    step.1.overlaps(&Cuboid {
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    })
}

fn part2_using_part1(steps: &[RebootStep]) -> u64 {
    let mut steps = steps.to_vec();
    steps.retain(is_initialization_step);
    part2(&steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SMALL_INPUT: &'static str = r"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
            .trim();
    }

    const LARGE_INPUT: &str = include_str!("sample/day22.txt");

    #[test]
    fn test_part1() {
        let input = input_generator(&SMALL_INPUT);
        assert_eq!(part1(&input), 39);
    }

    #[test]
    fn test_cuboid_split_x() {
        let cuboid = Cuboid {
            x: (10, 20),
            y: (1, 1),
            z: (2, 2),
        };
        assert_eq!(cuboid.split_x(10), Either::Left([cuboid]));
        assert_eq!(
            cuboid.split_x(11),
            Either::Right([
                Cuboid {
                    x: (10, 10),
                    ..cuboid
                },
                Cuboid {
                    x: (11, 20),
                    ..cuboid
                }
            ])
        );
        assert_eq!(
            cuboid.split_x(19),
            Either::Right([
                Cuboid {
                    x: (10, 18),
                    ..cuboid
                },
                Cuboid {
                    x: (19, 20),
                    ..cuboid
                }
            ])
        );
        assert_eq!(
            cuboid.split_x(20),
            Either::Right([
                Cuboid {
                    x: (10, 19),
                    ..cuboid
                },
                Cuboid {
                    x: (20, 20),
                    ..cuboid
                }
            ])
        );
        assert_eq!(cuboid.split_x(21), Either::Left([cuboid]));
    }

    #[test]
    fn test_cuboid_split() {
        let cuboid = Cuboid {
            x: (0, 10),
            y: (20, 30),
            z: (40, 50),
        };

        // Split in middle of X, Y and Z
        assert_eq!(
            cuboid.split(Vector3D::new(5, 25, 45)).collect::<Vec<_>>(),
            vec![
                Cuboid {
                    x: (0, 4),
                    y: (20, 24),
                    z: (40, 44)
                },
                Cuboid {
                    x: (0, 4),
                    y: (20, 24),
                    z: (45, 50)
                },
                Cuboid {
                    x: (0, 4),
                    y: (25, 30),
                    z: (40, 44)
                },
                Cuboid {
                    x: (0, 4),
                    y: (25, 30),
                    z: (45, 50)
                },
                Cuboid {
                    x: (5, 10),
                    y: (20, 24),
                    z: (40, 44)
                },
                Cuboid {
                    x: (5, 10),
                    y: (20, 24),
                    z: (45, 50)
                },
                Cuboid {
                    x: (5, 10),
                    y: (25, 30),
                    z: (40, 44)
                },
                Cuboid {
                    x: (5, 10),
                    y: (25, 30),
                    z: (45, 50)
                }
            ]
        );

        // Split in middle of X, split at end of Y, don't split Z
        assert_eq!(
            cuboid.split(Vector3D::new(5, 30, 60)).collect::<Vec<_>>(),
            vec![
                Cuboid {
                    x: (0, 4),
                    y: (20, 29),
                    z: (40, 50)
                },
                Cuboid {
                    x: (0, 4),
                    y: (30, 30),
                    z: (40, 50)
                },
                Cuboid {
                    x: (5, 10),
                    y: (20, 29),
                    z: (40, 50)
                },
                Cuboid {
                    x: (5, 10),
                    y: (30, 30),
                    z: (40, 50)
                }
            ]
        );
    }

    #[test]
    fn test_part2() {
        let input = input_generator(LARGE_INPUT);
        assert_eq!(part2(&input), 2758514936282235);
    }

    #[test]
    fn test_part1_using_part2() {
        let input = input_generator(&SMALL_INPUT);
        assert_eq!(part2_using_part1(&input), 39);

        let input = input_generator(LARGE_INPUT);
        assert_eq!(part2_using_part1(&input), 474140);
    }
}
