use std::collections::HashMap;

pub type RuleMap = HashMap<(char, char), char>;
pub type Input = (Vec<char>, RuleMap);

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let template = template.chars().collect();
    let rules = rules
        .lines()
        .map(|line| {
            let (pair, result) = line.split_once(" -> ").unwrap();
            let [left, right]: [char; 2] = pair.chars().collect::<Vec<_>>().try_into().unwrap();
            let result = result.chars().next().unwrap();
            ((left, right), result)
        })
        .collect();
    (template, rules)
}

fn step(polymer: &[char], rules: &RuleMap) -> Vec<char> {
    polymer
        .windows(2)
        .flat_map(|pair| {
            let (left, right) = (pair[0], pair[1]);
            // Each possible pair MUST have a rule
            let result = *rules.get(&(left, right)).unwrap();
            [left, result]
        })
        .chain(polymer.last().cloned())
        .collect()
}

fn solve_part1((template, rules): &Input, steps: usize) -> u64 {
    let mut polymer = template.clone();
    for _ in 1..=steps {
        polymer = step(&polymer, rules);
    }
    let mut counts = HashMap::<char, u64>::new();
    for &c in polymer.iter() {
        *counts.entry(c).or_default() += 1;
    }
    let min_count = *counts.values().min().unwrap();
    let max_count = *counts.values().max().unwrap();
    max_count - min_count
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> u64 {
    solve_part1(input, 10)
}

type PairCounts = HashMap<(char, char), u64>;
type ElementCounts = HashMap<char, u64>;

fn make_pair_counts(polymer: &[char]) -> PairCounts {
    let mut counts = PairCounts::new();
    for pair in polymer.windows(2) {
        let (left, right) = (pair[0], pair[1]);
        *counts.entry((left, right)).or_default() += 1;
    }
    counts
}

fn step_part2(counts: &PairCounts, rules: &RuleMap) -> PairCounts {
    let mut new_counts = PairCounts::new();
    for (&(left, right), count) in counts {
        // Each possible pair MUST have a rule
        let result = *rules.get(&(left, right)).unwrap();
        *new_counts.entry((left, result)).or_default() += count;
        *new_counts.entry((result, right)).or_default() += count;
    }
    new_counts
}

fn count_elements(pair_counts: &PairCounts, last_element: char) -> ElementCounts {
    let mut counts = ElementCounts::new();
    for (&(left, _), count) in pair_counts {
        // Each element appears in 2 pairs, except for the first and last pair of the polymer
        // Count only the first element of the pair, and then compensate for the last one
        *counts.entry(left).or_default() += count;
    }
    // The last element never changes, so we add it separately
    *counts.entry(last_element).or_default() += 1;
    counts
}

fn solve_part2((template, rules): &Input, steps: usize) -> u64 {
    let mut pair_counts = make_pair_counts(template);
    for _ in 1..=steps {
        pair_counts = step_part2(&pair_counts, rules);
    }
    let counts = count_elements(&pair_counts, *template.last().unwrap());
    let min_count = *counts.values().min().unwrap();
    let max_count = *counts.values().max().unwrap();
    max_count - min_count
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> u64 {
    solve_part2(input, 40)
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
CN -> C"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 2188189693529);
    }

    #[test]
    fn test_part1_using_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(solve_part2(&input, 10), 1588);
    }
}
