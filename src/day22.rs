use std::ops::RangeInclusive;

use lazy_static::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Cuboid {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
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
                x: captures[2].parse().unwrap()..=captures[3].parse().unwrap(),
                y: captures[4].parse().unwrap()..=captures[5].parse().unwrap(),
                z: captures[6].parse().unwrap()..=captures[7].parse().unwrap(),
            };
            RebootStep(on, cuboid)
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &[RebootStep]) -> i32 {
    todo!()
}

#[aoc(day22, part2)]
pub fn part2(input: &[RebootStep]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
            .trim();
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
