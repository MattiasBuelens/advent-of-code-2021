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
    let gamma_bits = (0..nb_bits).map(|i| {
        let nb_ones = input.iter().map(|number| number[i]).filter(|bit| *bit).count();
        nb_ones >= (nb_inputs / 2)
    }).collect::<Vec<_>>();
    let gamma = number_from_bits(&gamma_bits);
    let all_ones = (1 << nb_bits) - 1;
    let epsilon = all_ones - gamma;
    gamma * epsilon
}

fn number_from_bits(bits: &[bool]) -> i32 {
    bits.iter().rev().enumerate().map(|(pos, bit)| (*bit as i32) << pos).sum()
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
