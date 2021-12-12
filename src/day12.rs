use std::collections::VecDeque;

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

#[aoc(day12, part1)]
pub fn part1(input: &[(String, String)]) -> usize {
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
            // Small caves can only be visited once
            if next_cave.chars().any(|c| c.is_ascii_lowercase()) && path.contains(next_cave) {
                continue;
            }
            // Extend current path with next cave
            let mut new_path = path.clone();
            new_path.push(next_cave.clone());
            queue.push_back(new_path);
        }
    }
    dbg!(&finished_paths);
    finished_paths.len()
}

#[aoc(day12, part2)]
pub fn part2(input: &[(String, String)]) -> i32 {
    todo!()
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
        assert_eq!(part2(&input), 0);
    }
}
