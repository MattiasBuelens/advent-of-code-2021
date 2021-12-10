#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|s| parse(s))
        .map(|result| match result {
            ParseResult::Incomplete(_) => 0,
            ParseResult::Error(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("unexpected char {}", c),
            },
        })
        .sum()
}

enum ParseResult {
    Incomplete(Vec<char>),
    Error(char),
}

fn parse(line: &str) -> ParseResult {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match (stack.last(), c) {
            (_, '(') => stack.push(')'),
            (_, '[') => stack.push(']'),
            (_, '{') => stack.push('}'),
            (_, '<') => stack.push('>'),
            (Some(&last_close), close) if last_close == close => {
                stack.pop().unwrap();
            }
            (_, c) => return ParseResult::Error(c),
        }
    }
    stack.reverse();
    ParseResult::Incomplete(stack)
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> i64 {
    let mut scores = input
        .iter()
        .map(|s| parse(s))
        .filter_map(|result| match result {
            ParseResult::Incomplete(completion) => Some(autocomplete_score(&completion)),
            ParseResult::Error(_) => None,
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    assert_eq!(scores.len() % 2, 1, "must have odd number of scores");
    scores[scores.len() / 2]
}

fn autocomplete_score(completion: &[char]) -> i64 {
    let mut score = 0;
    for &c in completion {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            c => panic!("unexpected char {}", c),
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 288957);
    }
}
