use std::collections::HashSet;

type SegmentDisplay = HashSet<char>;

#[derive(Debug)]
pub struct Entry {
    patterns: [SegmentDisplay; 10],
    output: [SegmentDisplay; 4],
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();
            let patterns = patterns
                .split(' ')
                .map(|pattern| pattern.chars().collect())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let output = output
                .split(' ')
                .map(|pattern| pattern.chars().collect())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Entry { patterns, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Entry]) -> usize {
    input
        .iter()
        .map(|entry| {
            entry
                .output
                .iter()
                .filter(|display| matches!(display.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Entry]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | cgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
