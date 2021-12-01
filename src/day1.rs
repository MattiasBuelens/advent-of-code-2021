#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|&window| {
        let [prev, curr] = <&[i32; 2]>::try_from(window).unwrap();
        curr > prev
    }).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}
