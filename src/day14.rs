use std::collections::HashMap;

pub type RuleMap = HashMap<(char, char), char>;
pub type Input = (Vec<char>, RuleMap);

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.chars().collect();
    let rules = rules.lines().map(|line| {
        let (pair, result) = line.split_once(" -> ").unwrap();
        let [left, right]: [char; 2] = pair.chars().collect::<Vec<_>>().try_into().unwrap();
        let result = result.chars().next().unwrap();
        ((left, right), result)
    }).collect();
    (template, rules)
}

fn step(polymer: &[char], rules: &RuleMap) -> Vec<char> {
    polymer.windows(2).flat_map(|pair| {
        let (left, right) = (pair[0], pair[1]);
        if let Some(&result) = rules.get(&(left, right)) {
            vec![left, result]
        } else {
            vec![left]
        }
    }).chain(polymer.last().cloned()).collect()
}

#[aoc(day14, part1)]
pub fn part1((template, rules): &Input) -> usize {
    let mut polymer = template.clone();
    for _ in 1..=10 {
        polymer = step(&polymer, rules);
    }
    let mut counts = HashMap::<char, usize>::new();
    for &c in polymer.iter() {
        *counts.entry(c).or_default() += 1;
    }
    let min_count = *counts.values().min().unwrap();
    let max_count = *counts.values().max().unwrap();
    max_count - min_count
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C".trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
