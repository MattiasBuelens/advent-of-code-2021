#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    count_increments(input)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    let windows = input
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect::<Vec<_>>();
    count_increments(&windows)
}

fn count_increments(measurements: &[i32]) -> usize {
    measurements
        .windows(2)
        .filter(|&window| {
            let [prev, curr] = <&[i32; 2]>::try_from(window).unwrap();
            curr > prev
        })
        .count()
}
