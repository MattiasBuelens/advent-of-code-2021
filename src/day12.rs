use std::collections::{HashSet, VecDeque};

use multimap::MultiMap;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            (left.to_string(), right.to_string())
        })
        .collect()
}

type Path = Vec<String>;

pub fn solve(input: &[(String, String)], can_visit_small: fn(&String, &[String]) -> bool) -> usize {
    let mut edges = MultiMap::<String, String>::new();
    for (left, right) in input.to_vec() {
        edges.insert(left.clone(), right.clone());
        edges.insert(right, left);
    }
    let mut finished_paths = Vec::<Path>::new();
    let mut queue = VecDeque::<Path>::new();
    queue.push_back(vec!["start".to_string()]);
    while let Some(path) = queue.pop_front() {
        let current_cave = path.last().unwrap();
        if current_cave == "end" {
            finished_paths.push(path.clone());
        }
        for next_cave in edges.get_vec(current_cave).unwrap() {
            if is_small_cave(next_cave) && !can_visit_small(next_cave, &path) {
                continue;
            }
            // Extend current path with next cave
            let mut new_path = path.clone();
            new_path.push(next_cave.clone());
            queue.push_back(new_path);
        }
    }
    finished_paths.len()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().any(|c| c.is_ascii_lowercase())
}

#[aoc(day12, part1)]
pub fn part1(input: &[(String, String)]) -> usize {
    solve(input, |next_cave, path| {
        // Small caves can only be visited once
        !path.contains(next_cave)
    })
}

#[aoc(day12, part2)]
pub fn part2(input: &[(String, String)]) -> usize {
    solve(input, |next_cave, path| {
        // At most one small cave can be visited twice
        if !path.contains(next_cave) {
            return true;
        }
        // start and end can only be visited once
        if next_cave == "start" || next_cave == "end" {
            return false;
        }
        // Check if we have already visited any small cave more than once
        let small_caves = path
            .iter()
            .filter(|cave| is_small_cave(cave))
            .collect::<Vec<_>>();
        let small_caves_len = small_caves.len();
        let unique_small_caves = small_caves.into_iter().collect::<HashSet<_>>();
        small_caves_len == unique_small_caves.len()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 36);
    }
}
