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
            ParseResult::Incomplete => 0,
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
    Incomplete,
    Error(char),
}

fn parse(line: &str) -> ParseResult {
    let mut stack: Vec<char> = vec![];
    for c in line.chars() {
        match (stack.last(), c) {
            (_, '(' | '[' | '{' | '<') => stack.push(c),
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
                stack.pop().unwrap();
            }
            (_, c) => return ParseResult::Error(c),
        }
    }
    ParseResult::Incomplete
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> i32 {
    todo!()
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
        assert_eq!(part2(&input), 0);
    }
}
