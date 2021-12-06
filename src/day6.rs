use std::iter::repeat;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    let mut fishes = input.to_vec();
    for _i in 0..80 {
        step(&mut fishes);
    }
    fishes.len()
}

fn step(fishes: &mut Vec<u8>) {
    let mut new_fishes = 0usize;
    for fish in fishes.iter_mut() {
        *fish = match *fish {
            0 => {
                new_fishes += 1;
                6
            }
            n => n - 1,
        };
    }
    fishes.extend(repeat(8).take(new_fishes));
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"3,4,3,1,2".trim();
    }

    #[test]
    fn test_part1() {
        let mut fishes = input_generator(&TEST_INPUT);
        for _i in 0..18 {
            step(&mut fishes);
        }
        assert_eq!(fishes.len(), 26);
        for _i in 18..80 {
            step(&mut fishes);
        }
        assert_eq!(fishes.len(), 5934);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
