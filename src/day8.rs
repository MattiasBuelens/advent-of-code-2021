use std::collections::HashSet;

type SegmentDisplay = HashSet<char>;

#[derive(Debug, Clone)]
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
                .filter(|pattern| matches!(pattern.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Entry]) -> i32 {
    input.iter().map(decode).sum()
}

fn decode(entry: &Entry) -> i32 {
    let Entry { patterns, output } = entry.clone();

    let pattern1 = patterns
        .iter()
        .find(|pattern| pattern.len() == 2)
        .unwrap()
        .clone();
    let pattern4 = patterns
        .iter()
        .find(|pattern| pattern.len() == 4)
        .unwrap()
        .clone();
    let pattern7 = patterns
        .iter()
        .find(|pattern| pattern.len() == 3)
        .unwrap()
        .clone();
    let pattern8 = patterns
        .iter()
        .find(|pattern| pattern.len() == 7)
        .unwrap()
        .clone();

    // 7 - 1 = top middle segment
    let top_mid = *pattern7.difference(&pattern1).next().unwrap();

    // 0, 6, 9
    let patterns_with_6_segments = patterns
        .iter()
        .filter(|pattern| pattern.len() == 6)
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(patterns_with_6_segments.len(), 3);
    // 0 and 9 contain 1, but 6 doesn't
    let pattern6 = patterns_with_6_segments
        .iter()
        .find(|pattern| !pattern.is_superset(&pattern1))
        .unwrap()
        .clone();
    // 8 - 6 = top right segment
    let top_right = *pattern8.difference(&pattern6).next().unwrap();
    // 1 - top right = bottom right segment
    let bottom_right = *pattern1.iter().find(|&&c| c != top_right).unwrap();

    // 9 contains all of 4, but 0 doesn't contain all of 4
    let pattern9 = patterns_with_6_segments
        .iter()
        .find(|pattern| pattern.is_superset(&pattern4))
        .unwrap()
        .clone();
    let bottom_left = *pattern8.difference(&pattern9).next().unwrap();
    let pattern0 = patterns_with_6_segments
        .iter()
        .find(|pattern| ![&pattern6, &pattern9].contains(pattern))
        .unwrap()
        .clone();
    // 8 - 0 = middle segment
    let middle = *pattern8.difference(&pattern0).next().unwrap();

    // 4 - (middle, top right, bottom right) = top left segment
    let top_left = *pattern4
        .iter()
        .find(|c| ![middle, top_right, bottom_right].contains(c))
        .unwrap();

    // remaining = bottom middle segment
    let bottom_mid = *pattern0
        .iter()
        .find(|c| ![top_left, top_mid, top_right, bottom_left, bottom_right].contains(c))
        .unwrap();

    let pattern2 = SegmentDisplay::from([top_mid, top_right, middle, bottom_left, bottom_mid]);
    let pattern3 = SegmentDisplay::from([top_mid, top_right, middle, bottom_mid, bottom_right]);
    let pattern5 = SegmentDisplay::from([top_left, top_mid, middle, bottom_mid, bottom_right]);
    let lookup = [
        pattern0, pattern1, pattern2, pattern3, pattern4, pattern5, pattern6, pattern7, pattern8,
        pattern9,
    ];
    let mut value = 0;
    for output_display in output {
        value *= 10;
        value += lookup.iter().position(|p| p == &output_display).unwrap() as i32;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SMALL_INPUT: &'static str = r"
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
"
        .trim();
        static ref LARGE_INPUT: &'static str = r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
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
        let input = input_generator(&LARGE_INPUT);
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_decode_small() {
        let input = input_generator(&SMALL_INPUT);
        assert_eq!(decode(&input[0]), 5353);
    }

    #[test]
    fn test_decode_large() {
        let input = input_generator(&LARGE_INPUT);
        assert_eq!(
            input.iter().map(decode).collect::<Vec<_>>(),
            [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315]
        );
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&LARGE_INPUT);
        assert_eq!(part2(&input), 61229);
    }
}
