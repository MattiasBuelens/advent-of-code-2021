#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|c| c == '1').collect()
    }).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> i32 {
    let nb_inputs = input.len();
    let nb_bits = input[0].len();
    let gamma = (0..nb_bits).map(|i| {
        let nb_ones = input.iter().map(|number| number[i]).filter(|bit| *bit).count();
        let most_common_bit = nb_ones >= (nb_inputs / 2);
        let bit_pos = nb_bits - 1 - i;
        (most_common_bit as i32) << bit_pos
    }).sum::<i32>();
    let all_ones = (1 << nb_bits) - 1;
    let epsilon = all_ones - gamma;
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 198);
    }
}
