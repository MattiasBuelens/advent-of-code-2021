#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let min_pos = input.iter().min().unwrap().clone();
    let max_pos = input.iter().max().unwrap().clone();
    let mut best_pos = 0;
    let mut best_fuel = i32::MAX;
    'outer: for pos in min_pos..=max_pos {
        let mut fuel = 0;
        for &crab in input {
            if fuel >= best_fuel {
                continue 'outer;
            }
            fuel += (crab - pos).abs();
        }
        if fuel < best_fuel {
            best_pos = pos;
            best_fuel = fuel;
        }
    }
    best_fuel
}

#[aoc(day7, part2)]
pub fn part2(input: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part2(&input), 0);
    }
}
